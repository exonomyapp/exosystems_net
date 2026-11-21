"""Resolve the NVIDIA NIM API key from an operator-held file or environment.

The key is stored in `secrets/nvidia-nim-key.txt` at the repository root, which
is excluded from version control. An `NVIDIA_API_KEY` environment variable takes
precedence so the scripts also run where only an environment is available.
"""

import os
from pathlib import Path

KEY_FILE = Path(__file__).resolve().parents[2] / "secrets" / "nvidia-nim-key.txt"


def get_nvidia_key():
    key = os.environ.get("NVIDIA_API_KEY")
    if key:
        return key
    try:
        return KEY_FILE.read_text().strip()
    except FileNotFoundError:
        return None
