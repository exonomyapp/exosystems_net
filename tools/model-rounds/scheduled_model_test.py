#!/usr/bin/env python3
"""
Scheduled model verification task.
Runs every Monday and Thursday at 9am Berlin time.
Tests models from Alibaba MAAS, NVIDIA, and OpenCode Desktop.
Stores results in /home/exocrat/models.db with 3-strike inactive policy.
"""

import urllib.request
import json
import time
import sys
import sqlite3
import os
import uuid
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime

from nvidia_key import get_nvidia_key

# ============================================================
# CONFIGURATION
# ============================================================

# Use /home/exocrat/code/exosystems_net/tools/model-rounds/data as base directory
BASE_DIR = "/home/exocrat/code/exosystems_net/tools/model-rounds/data"

ALIBABA_API_KEY_FILE = "/home/exocrat/Downloads/Default Workspace-apiKey-1293585.csv"
ALIBABA_BASE_URL = "https://llm-3hfej6js6ima0lmv.ap-southeast-1.maas.aliyuncs.com/compatible-mode/v1"

NVIDIA_BASE_URL = "https://integrate.api.nvidia.com/v1"

OPENCODE_CONFIG_PATH = os.path.expanduser("~/.config/opencode/opencode.json")

DB_PATH = os.path.join(BASE_DIR, "models.db")

TEST_PROMPT = """Analyze this Python code and identify the bug:
def find_max(numbers):
    max_val = 0
    for n in numbers:
        if n > max_val:
            max_val = n
    return max_val
What's wrong? Provide the fix."""

TIMEOUT = 60
MAX_TOKENS = 200
MAX_CONSECUTIVE_FAILURES = 3

# Minimum context window, in tokens, for a model to enter the verification
# flow. Models with a smaller context window are skipped and logged.
MIN_CONTEXT_TOKENS = 262144

# At most this many models are tested at the same time within one round.
MAX_WORKERS = 8

# Figureless models published within this many days are admitted
# provisionally, with their release date recorded, and rechecked each round.
PROVISIONAL_DAYS = 90

# Public report page. The messaging alert carries this link and nothing else.
REPORT_URL = "https://exonomyapp.github.io/models/"

# Open model catalogue used to corroborate context figures that the
# provider catalogues do not publish.
MODELS_DEV_URL = "https://models.dev/api.json"

# Local copy of the corroborating catalogue, refreshed on each round.
CONTEXT_CACHE_FILE = os.path.join(BASE_DIR, "context_cache.json")

# ============================================================
# DATABASE SETUP
# ============================================================

def init_db():
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()
    cursor.execute("""
    CREATE TABLE IF NOT EXISTS model_tests (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        model_id TEXT NOT NULL,
        provider TEXT NOT NULL,
        test_prompt TEXT NOT NULL,
        success BOOLEAN NOT NULL,
        response_text TEXT,
        latency_ms INTEGER,
        tokens_used INTEGER,
        error_message TEXT,
        tested_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        test_batch INTEGER
    )
    """)
    cursor.execute("CREATE INDEX IF NOT EXISTS idx_model_tests_model_provider ON model_tests(model_id, provider)")
    cursor.execute("CREATE INDEX IF NOT EXISTS idx_model_tests_tested_at ON model_tests(tested_at)")

    cursor.execute("""
    CREATE TABLE IF NOT EXISTS model_registry (
        model_id TEXT PRIMARY KEY,
        provider TEXT NOT NULL,
        first_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        last_tested TIMESTAMP,
        last_success TIMESTAMP,
        consecutive_failures INTEGER DEFAULT 0,
        is_active BOOLEAN DEFAULT 1,
        total_tests INTEGER DEFAULT 0,
        successful_tests INTEGER DEFAULT 0,
        last_batch INTEGER DEFAULT 0,
        latency_p50 INTEGER DEFAULT 0,
        latency_p95 INTEGER DEFAULT 0,
        context_window INTEGER,
        context_source TEXT,
        failure_latency_1_ms INTEGER,
        failure_latency_2_ms INTEGER,
        failure_latency_3_ms INTEGER
    )
    """)
    cursor.execute("CREATE INDEX IF NOT EXISTS idx_model_registry_active ON model_registry(is_active, provider)")

    cursor.execute("""
    CREATE TABLE IF NOT EXISTS test_run_history (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        run_id TEXT UNIQUE NOT NULL,
        started_at REAL NOT NULL,
        completed_at REAL,
        provider TEXT NOT NULL,
        models_tested INTEGER DEFAULT 0,
        models_successful INTEGER DEFAULT 0,
        models_failed INTEGER DEFAULT 0,
        new_models_added INTEGER DEFAULT 0,
        models_flagged_inactive INTEGER DEFAULT 0,
        total_latency_ms INTEGER DEFAULT 0,
        avg_latency_ms REAL DEFAULT 0,
        p50_latency_ms REAL DEFAULT 0,
        p95_latency_ms REAL DEFAULT 0,
        batch INTEGER DEFAULT 0,
        error_summary TEXT,
        notes TEXT
    )
    """)
    cursor.execute("CREATE INDEX IF NOT EXISTS idx_test_run_history_run_id ON test_run_history(run_id)")
    cursor.execute("CREATE INDEX IF NOT EXISTS idx_test_run_history_provider ON test_run_history(provider)")
    cursor.execute("CREATE INDEX IF NOT EXISTS idx_test_run_history_started_at ON test_run_history(started_at)")
    conn.commit()
    return conn


# ============================================================
# HELPER FUNCTIONS
# ============================================================

def get_alibaba_key():
    try:
        with open(ALIBABA_API_KEY_FILE, 'r') as f:
            content = f.read()
            lines = content.strip().split('\n')
            for line in lines:
                if line.startswith('apiKey'):
                    return line.split(',', 1)[1].strip()
    except Exception as e:
        print(f"Error reading Alibaba key: {e}")
    return None


def test_model(base_url, headers, model_id, prompt=TEST_PROMPT, max_tokens=MAX_TOKENS, timeout=TIMEOUT):
    url = f"{base_url}/chat/completions"
    payload = {"model": model_id, "messages": [{"role": "user", "content": prompt}], "max_tokens": max_tokens, "temperature": 0.3, "stream": False}
    data = json.dumps(payload).encode()
    req = urllib.request.Request(url, data=data, headers=headers, method="POST")
    try:
        start = time.time()
        with urllib.request.urlopen(req, timeout=timeout) as response:
            latency = int((time.time() - start) * 1000)
            if response.status == 200:
                result = json.loads(response.read().decode())
                content = result.get("choices", [{}])[0].get("message", {}).get("content", "")
                usage = result.get("usage", {})
                return True, content.strip(), latency, usage
            else:
                err = response.read().decode()
                return False, f"HTTP {response.status}: {err[:300]}", 0, {}
    except Exception as e:
        return False, f"{type(e).__name__}: {e}", 0, {}


def fetch_models(base_url, headers):
    url = f"{base_url}/models"
    req = urllib.request.Request(url, headers=headers)
    try:
        with urllib.request.urlopen(req, timeout=30) as response:
            data = json.loads(response.read().decode())
            return [m.get("id") for m in data.get("data", [])]
    except Exception as e:
        print(f"Error fetching models: {e}")
        return []


def fetch_alibaba_context_map():
    """Return a mapping of Alibaba model id to (context window, published date).

    The mapping comes from the workspace model catalogue, which publishes a
    context window and a publication date for each model. Models absent from
    the catalogue map to nothing. Returns None when the catalogue itself is
    unreachable.
    """
    catalog_base = ALIBABA_BASE_URL.replace("/compatible-mode/v1", "")
    context_map = {}
    page = 1
    try:
        while True:
            url = f"{catalog_base}/api/v1/models?page_no={page}&page_size=50"
            req = urllib.request.Request(url, headers={"Authorization": f"Bearer {get_alibaba_key()}"})
            with urllib.request.urlopen(req, timeout=30) as response:
                output = json.loads(response.read().decode()).get("output", {})
            entries = output.get("models", [])
            if not entries:
                break
            for entry in entries:
                info = entry.get("model_info", {}) or {}
                context_map[entry.get("model")] = (info.get("context_window"), entry.get("published_time"))
            if len(context_map) >= output.get("total", 0):
                break
            page += 1
        return context_map
    except Exception as e:
        print(f"Error fetching Alibaba context catalog: {e}")
        return None


def fetch_models_dev_entries():
    """Return corroborating context figures from the open model catalogue.

    The result maps each catalogue model id to a list of
    (catalogue provider, context window) pairs. A local copy is kept so a
    round can proceed when the network copy is unreachable.
    """
    try:
        req = urllib.request.Request(MODELS_DEV_URL, headers={"User-Agent": "models/1.0"})
        with urllib.request.urlopen(req, timeout=90) as response:
            data = json.loads(response.read().decode())
        with open(CONTEXT_CACHE_FILE, "w") as f:
            json.dump({"fetched_at": time.time(), "catalogue": data}, f)
    except Exception as e:
        print(f"Open catalogue unreachable ({e}); trying local copy.")
        try:
            with open(CONTEXT_CACHE_FILE) as f:
                data = json.load(f).get("catalogue", {})
        except Exception:
            return {}
    entries = {}
    for provider_key, provider in (data or {}).items():
        for model_id, model in ((provider or {}).get("models") or {}).items():
            context = ((model or {}).get("limit") or {}).get("context")
            entries.setdefault(model_id, []).append((provider_key, context, (model or {}).get("release_date")))
    return entries


def model_age_days(date_value):
    """Return the age in days of a catalogue date, or None when unparseable."""
    if not date_value:
        return None
    for fmt in ("%Y-%m-%d %H:%M:%S", "%Y-%m-%d"):
        try:
            parsed = datetime.strptime(str(date_value)[:19], fmt)
            return (datetime.now() - parsed).days
        except Exception:
            continue
    return None


def verify_context(provider_name, model_id, alibaba_map, dev_entries):
    """Return (standing, context, source) for one model.

    Standing is "admitted" when a corroborated context window meets
    MIN_CONTEXT_TOKENS, "small" when a corroborated window falls below
    it, "provisional" when no figure exists but the model was published
    within PROVISIONAL_DAYS, and "unverified" otherwise. "admitted" and
    "provisional" models enter the flow; the rest are held out.
    """
    if provider_name == "alibaba_maas" and alibaba_map is not None:
        found = alibaba_map.get(model_id)
        if found is not None:
            context, published = found
            if context is not None:
                if context >= MIN_CONTEXT_TOKENS:
                    return "admitted", context, "alibaba-catalogue"
                return "small", context, "alibaba-catalogue"
            age = model_age_days(published)
            if age is not None and age <= PROVISIONAL_DAYS:
                return "provisional", None, f"provisional:released {str(published)[:10]}"
    family = {"alibaba_maas": "alibaba", "nvidia": "nvidia"}.get(provider_name, "")
    matches = []
    for wanted in (model_id, model_id.split("/")[-1]):
        for key, context, released in dev_entries.get(wanted, []):
            if not family or key == family or key.startswith(family + "-") or key.startswith(family + "_"):
                matches.append((key, context, released))
        if matches:
            break
    if not matches and family:
        seen = set()
        for wanted in (model_id, model_id.split("/")[-1]):
            for key, context, released in dev_entries.get(wanted, []):
                if key not in seen:
                    seen.add(key)
                    matches.append((key, context, released))
    known = [(key, context) for key, context, _ in matches if context is not None]
    if known:
        smallest = min(context for _, context in known)
        source = "models.dev:" + known[0][0]
        if smallest >= MIN_CONTEXT_TOKENS:
            return "admitted", smallest, source
        return "small", smallest, source
    dates = [released for _, _, released in matches if released]
    if dates:
        newest = max(str(d)[:10] for d in dates)
        if (model_age_days(newest) or 10 ** 9) <= PROVISIONAL_DAYS:
            return "provisional", None, f"provisional:released {newest}"
    if not matches:
        return "unverified", None, None
    return "unverified", None, "models.dev:figureless"


def gate_by_context(provider_name, model_ids, alibaba_map, dev_entries):
    """Split model ids into admitted and held-out lists.

    Each held-out entry carries its standing and corroborated figure:
    "small" means a confirmed window below the threshold, "unverified"
    means no source confirms a window and the model is not a recent
    publication. "admitted" covers confirmed windows and provisional
    recent publications, which are rechecked every round.
    """
    admitted = []
    held_out = []
    for model_id in model_ids:
        standing, context, source = verify_context(provider_name, model_id, alibaba_map, dev_entries)
        if standing in ("admitted", "provisional"):
            admitted.append((model_id, context, source))
        else:
            held_out.append((model_id, standing, context, source))
    return admitted, held_out


def get_next_batch_number(conn):
    cursor = conn.cursor()
    cursor.execute("SELECT MAX(test_batch) FROM model_tests")
    result = cursor.fetchone()
    return (result[0] or 0) + 1


def update_model_registry(conn, model_id, provider, success, tested_at, batch, latencies=None, context=None, source=None, this_latency=0):
    cursor = conn.cursor()

    # Get current registry entry
    cursor.execute("SELECT consecutive_failures, total_tests, successful_tests, last_success, latency_p50, latency_p95, failure_latency_1_ms, failure_latency_2_ms, failure_latency_3_ms FROM model_registry WHERE model_id = ? AND provider = ?", (model_id, provider))
    row = cursor.fetchone()

    is_new = row is None
    was_active = True if is_new else (row[0] or 0) < MAX_CONSECUTIVE_FAILURES

    if row:
        consecutive_failures, total_tests, successful_tests, last_success, lat_p50, lat_p95, fail_1, fail_2, fail_3 = row
        total_tests += 1
        if success:
            successful_tests += 1
            consecutive_failures = 0
            last_success = tested_at
        else:
            consecutive_failures += 1
            last_success = None
            fail_1, fail_2, fail_3 = this_latency, fail_1, fail_2
        
        # Update latency percentiles
        all_latencies = []
        if lat_p50 is not None:
            all_latencies.append(lat_p50)
        if lat_p95 is not None:
            all_latencies.append(lat_p95)
        if latencies:
            all_latencies.extend(latencies)
        
        if all_latencies:
            sorted_lat = sorted(all_latencies)
            n = len(sorted_lat)
            p50_idx = int(0.5 * (n - 1))
            p95_idx = int(0.95 * (n - 1))
            new_p50 = sorted_lat[p50_idx]
            new_p95 = sorted_lat[p95_idx]
        else:
            new_p50 = lat_p50 or 0
            new_p95 = lat_p95 or 0

        is_active = 1 if consecutive_failures < MAX_CONSECUTIVE_FAILURES else 0

        cursor.execute("""
            UPDATE model_registry SET
                last_tested = ?,
                last_success = ?,
                consecutive_failures = ?,
                is_active = ?,
                total_tests = ?,
                successful_tests = ?,
                latency_p50 = ?,
                latency_p95 = ?,
                last_batch = ?,
                context_window = ?,
                context_source = ?,
                failure_latency_1_ms = ?,
                failure_latency_2_ms = ?,
                failure_latency_3_ms = ?
            WHERE model_id = ? AND provider = ?
        """, (tested_at, last_success, consecutive_failures, is_active, total_tests, successful_tests, new_p50, new_p95, batch, context, source, fail_1, fail_2, fail_3, model_id, provider))
    else:
        # New model - insert into registry
        consecutive_failures = 0 if success else 1
        is_active = 1 if consecutive_failures < MAX_CONSECUTIVE_FAILURES else 0
        total_tests = 1
        successful_tests = 1 if success else 0
        last_success = tested_at if success else None

        cursor.execute("""
            INSERT INTO model_registry (model_id, provider, first_seen, last_tested, last_success,
                                          consecutive_failures, is_active, total_tests, successful_tests, last_batch, latency_p50, latency_p95, context_window, context_source,
                                          failure_latency_1_ms, failure_latency_2_ms, failure_latency_3_ms)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """, (model_id, provider, tested_at, tested_at, last_success, consecutive_failures, is_active, total_tests, successful_tests, batch, 0 if not latencies else (latencies[0] if latencies else 0), 0 if not latencies else (latencies[-1] if latencies else 0), context, source,
              this_latency if not success else None, None, None))

    conn.commit()
    became_inactive = was_active and (is_active == 0)
    return is_new, became_inactive


def get_active_models(conn, provider):
    """Get list of currently active models for a provider"""
    cursor = conn.cursor()
    cursor.execute("SELECT model_id FROM model_registry WHERE provider = ? AND is_active = 1", (provider,))
    return [row[0] for row in cursor.fetchall()]


def compute_latency_percentiles(latencies):
    """Compute p50, p95 from a list of latency values in ms"""
    if not latencies:
        return 0, 0
    sorted_lat = sorted(latencies)
    n = len(sorted_lat)
    p50_idx = int(0.5 * (n - 1))
    p95_idx = int(0.95 * (n - 1))
    return sorted_lat[p50_idx], sorted_lat[p95_idx]


# ============================================================
# PROVIDER TESTS
# ============================================================

def test_provider(conn, provider_name, base_url, headers, fetch_new_models=True):
    print(f"\n{'='*60}")
    print(f"TESTING {provider_name.upper()} MODELS")
    print(f"{'='*60}")

    cursor = conn.cursor()

    # Resume an unfinished round for this provider when one exists, so an
    # interrupted run continues instead of starting over. Otherwise open a
    # new round: the history row is written first with no completion time,
    # which marks the round as running.
    cursor.execute("""
        SELECT run_id, batch FROM test_run_history
        WHERE provider = ? AND completed_at IS NULL
        ORDER BY started_at DESC LIMIT 1
    """, (provider_name,))
    resumed = cursor.fetchone()
    if resumed:
        run_id, batch = resumed
        print(f"Resuming unfinished round {run_id} (batch {batch})")
        new_round = False
    else:
        batch = get_next_batch_number(conn)
        run_id = f"{provider_name}_b{batch}_{int(time.time())}"
        cursor.execute("""
            INSERT INTO test_run_history (run_id, started_at, completed_at, provider, batch, notes)
            VALUES (?, ?, NULL, ?, ?, ?)
        """, (run_id, time.time(), provider_name, batch, f"Round {batch} running"))
        conn.commit()
        new_round = True

    # Fetch current models from API
    api_models = fetch_models(base_url, headers)
    print(f"API returned {len(api_models)} models")

    # Verified context figures: the serving catalogue first, the open
    # catalogue as corroboration. A model enters the flow only with a
    # confirmed window of at least MIN_CONTEXT_TOKENS.
    alibaba_map = fetch_alibaba_context_map() if provider_name == "alibaba_maas" else {}
    if provider_name == "alibaba_maas" and not alibaba_map:
        print("Alibaba context catalogue unreachable and no corroboration available; skipping this provider run.")
        return []
    dev_entries = fetch_models_dev_entries()
    admitted, held_out = gate_by_context(provider_name, api_models, alibaba_map, dev_entries)
    admitted_context = {model: (context, source) for model, context, source in admitted}
    print(f"Context check admitted {len(admitted)} models and held out {len(held_out)}")
    for model_id, standing, context, source in sorted(held_out):
        print(f"  held out: {model_id} ({standing}, {context})")

    # Get existing active models from registry
    active_models = get_active_models(conn, provider_name)
    print(f"Registry has {len(active_models)} active models")

    # The test set joins active registry models with newly seen catalogue
    # models. Registry models admitted earlier keep their stored figures;
    # anything the check holds out stays out.
    models_to_test = {}
    for model in active_models:
        standing, context, source = verify_context(provider_name, model, alibaba_map, dev_entries)
        if standing in ("admitted", "provisional"):
            models_to_test[model] = (context, source)
        else:
            print(f"  held out registry model: {model} ({standing}, {context})")
    newly_added = []
    if fetch_new_models:
        cursor.execute("SELECT model_id FROM model_registry WHERE provider = ?", (provider_name,))
        known_models = {row[0] for row in cursor.fetchall()}
        for model, context, source in admitted:
            if model not in known_models and model not in models_to_test:
                models_to_test[model] = (context, source)
                newly_added.append(model)
        if newly_added:
            print(f"Found {len(newly_added)} new models: {newly_added[:10]}{'...' if len(newly_added) > 10 else ''}")

    # Skip models this round already tested, so a resumed round continues
    # where the interruption left off.
    cursor.execute("SELECT model_id FROM model_tests WHERE provider = ? AND test_batch = ?", (provider_name, batch))
    already_tested = {row[0] for row in cursor.fetchall()}
    if already_tested:
        print(f"Skipping {len(already_tested)} models already tested in batch {batch}")
    queue = sorted(set(models_to_test) - already_tested)
    print(f"Testing {len(queue)} models in batch {batch} with {MAX_WORKERS} workers...")

    results = []
    transitioned_inactive = []

    def test_one(model):
        worker = sqlite3.connect(DB_PATH, timeout=30.0)
        worker.execute("PRAGMA busy_timeout = 5000")
        try:
            success, content, latency, usage = test_model(base_url, headers, model)
            tested_at = datetime.now().isoformat()
            context, source = models_to_test[model]
            worker_cursor = worker.cursor()
            worker_cursor.execute("""
                INSERT INTO model_tests (model_id, provider, test_prompt, success, response_text, latency_ms, tokens_used, error_message, test_batch)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            """, (model, provider_name, TEST_PROMPT, success, content if success else None, latency if success else 0, usage.get("total_tokens", 0) if success else 0, content if not success else None, batch))
            is_new, became_inactive = update_model_registry(worker, model, provider_name, success, tested_at, batch, [latency] if success else [], context, source, latency)
            worker.commit()
            tokens = usage.get("total_tokens", 0) if success else 0
            return (model, success, latency, tokens, is_new, became_inactive)
        finally:
            worker.close()

    latencies = []
    done = 0
    total = len(queue)
    worker_errors = []
    with ThreadPoolExecutor(max_workers=MAX_WORKERS) as pool:
        futures = {pool.submit(test_one, model): model for model in queue}
        for future in as_completed(futures):
            try:
                model, success, latency, tokens, is_new, became_inactive = future.result()
            except Exception as e:
                worker_errors.append(f"{futures[future]}: {e}")
                print(f"  worker failed for {futures[future]}: {e}")
                continue
            done += 1
            if success:
                latencies.append(latency)
                print(f"[{done}/{total}] {model}: OK ({latency}ms, {tokens} tokens)")
            else:
                print(f"[{done}/{total}] {model}: FAILED")
            if became_inactive:
                transitioned_inactive.append(model)
            results.append((model, success, latency, tokens))
    if worker_errors:
        print(f"{len(worker_errors)} workers failed; leaving round {run_id} open for resume.")
        raise RuntimeError(f"Interrupted round {run_id}: " + "; ".join(worker_errors[:5]))

    # --- Close out this provider's round ---
    models_tested = len(results)
    models_successful = sum(1 for r in results if r[1])
    models_failed = models_tested - models_successful

    total_latency_ms = sum(latencies)
    avg_latency_ms = total_latency_ms / models_tested if models_tested else 0
    p50, p95 = compute_latency_percentiles(latencies)

    cursor.execute("""
        UPDATE test_run_history SET
            completed_at = ?,
            models_tested = models_tested + ?,
            models_successful = models_successful + ?,
            models_failed = models_failed + ?,
            new_models_added = ?,
            models_flagged_inactive = ?,
            total_latency_ms = total_latency_ms + ?,
            avg_latency_ms = ?,
            p50_latency_ms = ?,
            p95_latency_ms = ?,
            notes = ?
        WHERE run_id = ?
    """, (time.time(), models_tested, models_successful, models_failed, len(newly_added), len(transitioned_inactive), total_latency_ms, avg_latency_ms, p50, p95, f"Round {batch} finished", run_id))
    conn.commit()

    # --- Messaging alert: new models and newly inactive models only ---
    if newly_added or transitioned_inactive:
        bridge_url = os.environ.get("HERMES_BRIDGE_URL", "http://localhost:3099")
        user_chat_id = "48559151956170@lid"
        alert_lines = ["Model verification round finished"]
        if newly_added:
            alert_lines.append(f"New models: {len(newly_added)}")
            for model_id in sorted(newly_added)[:20]:
                alert_lines.append(f"  + {model_id}")
        if transitioned_inactive:
            alert_lines.append(f"Flagged inactive: {len(transitioned_inactive)}")
            for model_id in sorted(transitioned_inactive)[:20]:
                alert_lines.append(f"  - {model_id}")
        alert_lines.append(f"Full report: {REPORT_URL}")
        try:
            req = urllib.request.Request(
                f"{bridge_url}/send",
                data=json.dumps({"chatId": user_chat_id, "message": "\n".join(alert_lines)}).encode(),
                headers={"Content-Type": "application/json"},
                method="POST"
            )
            with urllib.request.urlopen(req, timeout=10) as response:
                print(f"WhatsApp alert sent: {response.read().decode()[:100]}")
        except Exception as e:
            print(f"Failed to send WhatsApp alert: {e}")
    else:
        print("Nothing new to announce; no alert sent.")

    return results


def test_alibaba_models(conn):
    api_key = get_alibaba_key()
    if not api_key:
        print("No Alibaba API key found")
        return
    headers = {"Authorization": f"Bearer {api_key}", "Content-Type": "application/json"}
    return test_provider(conn, "alibaba_maas", ALIBABA_BASE_URL, headers)


def test_nvidia_models(conn):
    api_key = get_nvidia_key()
    if not api_key:
        print("No NVIDIA API key found")
        return
    headers = {"Authorization": f"Bearer {api_key}", "Content-Type": "application/json"}
    return test_provider(conn, "nvidia", NVIDIA_BASE_URL, headers)


def test_opencode_models(conn):
    print(f"\n{'='*60}")
    print("TESTING OPENCODE DESKTOP MODELS")
    print(f"{'='*60}")

    if os.path.exists(OPENCODE_CONFIG_PATH):
        try:
            with open(OPENCODE_CONFIG_PATH, 'r') as f:
                config = json.load(f)
                print(f"OpenCode config found")
                model_list = config.get("models", [])
                if model_list:
                    print(f"Found {len(model_list)} configured models")
                    for m in model_list[:5]:
                        print(f"  - {m}")
                    # Test each configured model
                    for model in model_list:
                        print(f"  Testing: {model}")
                        success, content, latency, usage = test_model("http://localhost", {"Authorization": "Bearer dummy"}, model)
                        tested_at = datetime.now().isoformat()

                        if success:
                            tokens = usage.get("total_tokens", 0)
                            print(f"    OK ({latency}ms, {tokens} tokens)")
                        else:
                            print(f"    FAILED")

                        # Record test result
                        cursor = conn.cursor()
                        cursor.execute("""
                            INSERT INTO model_tests (model_id, provider, test_prompt, success, response_text, latency_ms, tokens_used, error_message, test_batch)
                            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                        """, (model, "opencode", TEST_PROMPT, success, content if success else None, latency if success else 0, usage.get("total_tokens", 0) if success else 0, content if not success else None, 0))

                        # Update registry
                        update_model_registry(conn, model, "opencode", success, tested_at, 0)
                        conn.commit()
                else:
                    print("No models configured in OpenCode config")
                    # List what's in the config
                    print(f"Config keys: {list(config.keys())}")
        except Exception as e:
            print(f"Error reading OpenCode config: {e}")
    else:
        print(f"OpenCode config not found at {OPENCODE_CONFIG_PATH}")
        print("To add OpenCode Desktop model testing:")
        print("1. Configure models in ~/.config/opencode/opencode.json")
        print("2. The config should have a 'models' list with model IDs")
        print("3. Run scheduled_model_test.py to test them")


# ============================================================
# MAIN
# ============================================================

def main():
    print(f"Starting scheduled model verification at {datetime.now().isoformat()}")
    print(f"Berlin time: {datetime.now().astimezone().strftime('%Y-%m-%d %H:%M:%S %Z')}")

    conn = init_db()

    try:
        test_alibaba_models(conn)
        test_nvidia_models(conn)
        test_opencode_models(conn)
    finally:
        conn.close()

    # Print summary
    conn = sqlite3.connect(DB_PATH)
    cursor = conn.cursor()
    cursor.execute("SELECT provider, COUNT(*), SUM(CASE WHEN is_active THEN 1 ELSE 0 END) FROM model_registry GROUP BY provider")
    print("\n" + "="*60)
    print("REGISTRY SUMMARY")
    print("="*60)
    for row in cursor.fetchall():
        print(f"  {row[0]}: {row[2]}/{row[1]} active")

    cursor.execute("SELECT provider, model_id, consecutive_failures FROM model_registry WHERE consecutive_failures >= 3 ORDER BY provider, consecutive_failures DESC")
    inactive = cursor.fetchall()
    if inactive:
        print("\nINACTIVE MODELS (3+ strikes):")
        for row in inactive:
            print(f"  {row[1]} ({row[0]}): {row[2]} strikes")

    # Print test run history summary
    cursor.execute("SELECT run_id, provider, models_tested, models_successful FROM test_run_history ORDER BY started_at DESC LIMIT 10")
    print("\nLATEST TEST RUNS:")
    for row in cursor.fetchall():
        print(f"  {row[0]} | {row[1]} | tested: {row[2]} | successful: {row[3]}")
    conn.close()

    print(f"\nCompleted at {datetime.now().isoformat()}")


if __name__ == "__main__":
    main()