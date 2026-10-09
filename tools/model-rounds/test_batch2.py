import urllib.request
import json
import time
import sys

with open('/home/exocrat/Downloads/Default Workspace-apiKey-1293585.csv', 'r') as f:
    content = f.read()
    lines = content.strip().split('\n')
    api_key = None
    for line in lines:
        if line.startswith('apiKey'):
            api_key = line.split(',', 1)[1].strip()
            break

base_url = "https://llm-3hfej6js6ima0lmv.ap-southeast-1.maas.aliyuncs.com/compatible-mode/v1"
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

# Additional models from the 172 list - focusing on large/untested models
batch2_models = [
    "qwen3.5-plus",
    "qwen3.6-plus",
    "qwen3.5-max",
    "kimi/kimi-k3",
    "kimi-k3",
    "qwen3-plus-character",
    "qwen-flash-character",
    "qwen-plus-character",
    "qwen-plus-latest",
    "qwen-plus-2025-09-11",
    "qwen-plus-2025-07-14",
    "qwen-plus-2025-04-28",
    "qwen-max",
    "qwen-max-2025-09-23",
    "qwen-vl-max",
    "qwen-vl-plus",
    "qwen-vl-plus-2025-09-23",
]

print(f"Testing {len(batch2_models)} models...")
for model in batch2_models:
    print(f"\nTesting: {model}")
    success, content, latency, usage = test_model(model, test_prompt)
    if success:
        tokens = usage.get("total_tokens", "?")
        print(f"  OK ({latency}ms, {tokens} tokens) - {content[:200]}")
    else:
        print(f"  FAILED - {content}")
