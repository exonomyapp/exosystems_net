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

# Top NVIDIA models (Batch 1: 10 largest/most capable)
nvidia_models_batch1 = [
    "nvidia/nemotron-3-ultra-550b-a55b",
    "nvidia/nemotron-4-340b-instruct",
    "nvidia/nemotron-3-super-120b-a12b",
    "nvidia/llama-3.1-nemotron-ultra-253b-v1",
    "nvidia/llama-3.1-nemotron-70b-instruct",
    "meta/llama-3.2-90b-vision-instruct",
    "mistralai/mistral-large-2-instruct",
    "mistralai/mixtral-8x22b-v0.1",
    "meta/codellama-70b",
    "z-ai/glm-5.3",
]

print(f"Testing {len(nvidia_models_batch1)} NVIDIA models (Batch 1)...")
results = []
for model in nvidia_models_batch1:
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
    """, (r[0], "nvidia", test_prompt, r[1], r[2] if r[1] else None, r[2] if r[1] else 0, r[3] if r[1] else 0, r[4], 4))
conn.commit()
conn.close()
print("\nNVIDIA Batch 1 results saved to models.db")