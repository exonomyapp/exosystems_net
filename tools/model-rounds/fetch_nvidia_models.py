import urllib.request
import json
import time
import sys

from nvidia_key import get_nvidia_key

api_key = get_nvidia_key()
if not api_key:
    print("No NVIDIA API key found")
    sys.exit(1)
base_url = "https://integrate.api.nvidia.com/v1"
headers = {"Authorization": f"Bearer {api_key}", "Content-Type": "application/json"}

url = f"{base_url}/models"
req = urllib.request.Request(url, headers=headers)
try:
    with urllib.request.urlopen(req, timeout=30) as response:
        data = json.loads(response.read().decode())
        models = [m.get("id") for m in data.get("data", [])]
        print(f"Found {len(models)} NVIDIA models")
        for m in models:
            print(f"  {m}")
except Exception as e:
    print(f"Error: {e}")