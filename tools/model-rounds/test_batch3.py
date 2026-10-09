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

# Remaining models from 172 list - multimodal, embedding, etc.
batch3_models = [
    "qwen3-8b",
    "qwen3-30b-a3b",
    "qwen3-235b-a22b-instruct-2507",
    "qwen3-235b-a22b-thinking-2507",
    "qwen2-7b-instruct",
    "qwen-turbo",
    "qwq-plus",
    "qwq-plus-2025-03-05",
    "qvq-max",
    "qwen-omni-turbo",
    "qwen3-omni-flash",
    "qwen3-omni-flash-realtime",
    "qwen3-vl-235b-a22b-instruct",
    "qwen3-vl-plus",
    "qwen3-vl-235b-a22b-thinking",
    "qwen-image-3.0-pro",
    "qwen-image-3.0",
    "qwen-image-2.0-pro",
    "qwen-image-2.0",
    "qwen-image-edit-max",
    "qwen-image-edit-plus",
    "text-embedding-v4",
    "text-embedding-v3",
    "qwen3-asr-flash",
    "qwen3-tts-flash",
]

print(f"Testing {len(batch3_models)} models...")
for model in batch3_models:
    print(f"\nTesting: {model}")
    success, content, latency, usage = test_model(model, test_prompt)
    if success:
        tokens = usage.get("total_tokens", "?")
        print(f"  OK ({latency}ms, {tokens} tokens) - {content[:200]}")
    else:
        print(f"  FAILED - {content}")
