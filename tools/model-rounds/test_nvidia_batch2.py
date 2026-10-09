import urllib.request
import json
import time
import sys
import sqlite3

api_key = "***REMOVED***"
base_url = "https://integrate.api.nvidia.com/v1"
headers = {"Authorization": f"Bearer {api_key}", "Content-Type": "application/json"}

def test_model(model_id, prompt, max_tokens=200, timeout=120):
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

test_prompt = """Analyze this Python code and identify the bug:
def find_max(numbers):
    max_val = 0
    for n in numbers:
        if n > max_val:
            max_val = n
    return max_val
What's wrong? Provide the fix."""

# NVIDIA Batch 2: Next 10 models that might work
nvidia_models_batch2 = [
    "nvidia/nemotron-3-nano-omni-30b-a3b-reasoning",
    "nvidia/nemotron-3.5-lightning-30b-a3b",
    "nvidia/nemotron-4-340b-reward",
    "nvidia/nemotron-nano-3-30b-a3b",
    "nvidia/nemotron-parse",
    "nvidia/nemotron-parse-2.0",
    "nvidia/neva-22b",
    "nvidia/nv-embedqa-mistral-7b-v2",
    "nvidia/nvclip",
    "nvidia/riva-translate-4b-instruct",
    "nvidia/riva-translate-4b-instruct-v2",
    "nvidia/vila",
    "meta/llama-3.2-11b-vision-instruct",
    "moonshotai/kimi-k2.6",
    "moonshotai/kimi-k3",
    "nv-mistralai/mistral-nemo-12b-instruct",
    "mistralai/mistral-7b-instruct-v0.3",
    "mistralai/codestral-22b-instruct-v0.1",
    "bigcode/starcoder2-15b",
    "ibm/granite-3.0-8b-instruct",
    "ibm/granite-34b-code-instruct",
    "google/gemma-4-31b-it",
    "deepseek-ai/deepseek-v4.1-flash",
    "openai/gpt-oss-20b",
    "writer/palmyra-creative-122b",
    "writer/palmyra-fin-70b-32k",
    "writer/palmyra-med-70b",
    "writer/palmyra-med-70b-32k",
    "zyphra/zamba2-7b-instruct",
    "z-ai/glm-5.3-flash",
]

# Test first 10
nvidia_models_batch2 = nvidia_models_batch2[:10]

print(f"Testing {len(nvidia_models_batch2)} NVIDIA models (Batch 2)...")
results = []
for model in nvidia_models_batch2:
    print(f"\nTesting: {model}")
    success, content, latency, usage = test_model(model, test_prompt)
    if success:
        tokens = usage.get("total_tokens", "?")
        print(f"  OK ({latency}ms, {tokens} tokens) - {content[:200]}")
        results.append((model, True, latency, tokens, None))
    else:
        print(f"  FAILED - {content}")
        results.append((model, False, 0, 0, content))

# Save to DB
conn = sqlite3.connect("/home/exocrat/models.db")
cursor = conn.cursor()
for r in results:
    cursor.execute("""
        INSERT INTO model_tests (model_id, provider, test_prompt, success, response_text, latency_ms, tokens_used, error_message, test_batch)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    """, (r[0], "nvidia", test_prompt, r[1], r[2] if r[1] else None, r[2] if r[1] else 0, r[3] if r[1] else 0, r[4], 5))
conn.commit()
conn.close()
print("\nNVIDIA Batch 2 results saved to models.db")