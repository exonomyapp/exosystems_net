#!/usr/bin/env python3
"""Dump the operational summary of models.db to a snapshot file.

Usage: python3 export_snapshot.py /path/to/snapshot.json
"""

import json
import sqlite3
import sys
from datetime import datetime, timezone

BASE_DIR = "/home/exocrat/code/exosystems_net/tools/model-rounds/data"
DB_PATH = f"{BASE_DIR}/models.db"


def main(path):
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    cur = conn.cursor()
    providers = [
        {"provider": r[0], "total": r[1], "active": r[2]}
        for r in cur.execute(
            "SELECT provider, COUNT(*), SUM(is_active) FROM model_registry GROUP BY provider"
        ).fetchall()
    ]
    rounds = [
        {
            "run_id": r["run_id"],
            "provider": r["provider"],
            "tested": r["models_tested"],
            "successful": r["models_successful"],
            "failed": r["models_failed"],
            "new": r["new_models_added"],
            "inactive": r["models_flagged_inactive"],
        }
        for r in cur.execute(
            "SELECT * FROM test_run_history ORDER BY started_at DESC LIMIT 20"
        ).fetchall()
    ]
    active = [
        {
            "model": r["model_id"],
            "provider": r["provider"],
            "context_window": r["context_window"],
            "last_tested": r["last_tested"],
        }
        for r in cur.execute(
            "SELECT model_id, provider, context_window, last_tested FROM model_registry WHERE is_active = 1 ORDER BY provider, model_id"
        ).fetchall()
    ]
    inactive = [
        {"model": r["model_id"], "provider": r["provider"], "failures": r["consecutive_failures"]}
        for r in cur.execute(
            "SELECT model_id, provider, consecutive_failures FROM model_registry WHERE is_active = 0 ORDER BY provider, model_id"
        ).fetchall()
    ]
    conn.close()
    snapshot = {
        "generated_at": datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC"),
        "providers": providers,
        "rounds": rounds,
        "active_models": active,
        "inactive_models": inactive,
    }
    with open(path, "w") as f:
        json.dump(snapshot, f, indent=2)
    print(f"Wrote {path}: {len(active)} active, {len(inactive)} inactive, {len(rounds)} rounds")


if __name__ == "__main__":
    main(sys.argv[1])
