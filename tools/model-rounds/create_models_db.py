import sqlite3
import json
from datetime import datetime

db_path = "/home/exocrat/models.db"
conn = sqlite3.connect(db_path)
cursor = conn.cursor()

# Create tables
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

cursor.execute("""
CREATE INDEX IF NOT EXISTS idx_model_provider ON model_tests(model_id, provider)
""")

cursor.execute("""
CREATE INDEX IF NOT EXISTS idx_tested_at ON model_tests(tested_at)
""")

conn.commit()
print("Database created at", db_path)

# Insert results from Batch 1 (Alibaba MAAS)
batch1_results = [
    ("qwen3-235b-a22b", "alibaba_maas", True, "OK", 5028, 285, None, 1),
    ("qwen3-235b-a22b-instruct-2507", "alibaba_maas", True, "OK", 1047, 233, None, 1),
    ("qwen3-235b-a22b-thinking-2507", "alibaba_maas", True, "OK", 15130, 4143, None, 1),
    ("qwen3.5-397b-a17b", "alibaba_maas", True, "OK", 6862, 2248, None, 1),
    ("qwen3.6-max-preview", "alibaba_maas", True, "OK", 4869, 2212, None, 1),
    ("qwen3.8-max", "alibaba_maas", True, "OK", 2016, 259, None, 1),
    ("qwen3.8-max-0902", "alibaba_maas", True, "OK", 3049, 316, None, 1),
    ("qwen3.7-max", "alibaba_maas", True, "OK", 2450, 259, None, 1),
    ("qwen3.7-max-preview", "alibaba_maas", True, "OK", 2579, 338, None, 1),
    ("glm-5.3", "alibaba_maas", True, "OK", 1366, 239, None, 1),
    ("glm-5.3-prime", "alibaba_maas", True, "OK", 2354, 271, None, 1),
    ("deepseek-v4-pro", "alibaba_maas", True, "OK", 2077, 275, None, 1),
    ("deepseek-v4-pro-0813", "alibaba_maas", True, "OK", 2586, 235, None, 1),
    ("deepseek-v3.2", "alibaba_maas", True, "OK", 1514, 241, None, 1),
    ("kimi-k2.7-code", "alibaba_maas", True, "OK", 3518, 258, None, 1),
    ("qwen3-coder-480b-a35b-instruct", "alibaba_maas", True, "OK", 2332, 318, None, 1),
    ("qwen3-coder-plus", "alibaba_maas", True, "OK", 2244, 233, None, 1),
    ("qwen3-coder-plus-2025-07-22", "alibaba_maas", True, "OK", 2562, 248, None, 1),
    ("qwen3-coder-plus-2025-09-23", "alibaba_maas", True, "OK", 1751, 230, None, 1),
    ("qwen3-coder-next", "alibaba_maas", True, "OK", 2427, 265, None, 1),
    ("qwen3-next-80b-a3b-thinking", "alibaba_maas", True, "OK", 17659, 4208, None, 1),
    ("qwen3-next-80b-a3b-instruct", "alibaba_maas", True, "OK", 1880, 265, None, 1),
    ("qwen3-30b-a3b-thinking-2507", "alibaba_maas", True, "OK", 27942, 4422, None, 1),
    ("qwen3-30b-a3b-instruct-2507", "alibaba_maas", True, "OK", 910, 265, None, 1),
]

# Batch 2
batch2_results = [
    ("qwen3.5-plus", "alibaba_maas", True, "OK", 10461, 560, None, 2),
    ("qwen3.6-plus", "alibaba_maas", True, "OK", 20483, 1213, None, 2),
    ("qwen3.5-max", "alibaba_maas", False, None, 0, 0, "HTTP 404: Not Found", 2),
    ("kimi/kimi-k3", "alibaba_maas", False, None, 0, 0, "HTTP 400: Bad Request", 2),
    ("kimi-k3", "alibaba_maas", False, None, 0, 0, "HTTP 400: Bad Request", 2),
    ("qwen3-plus-character", "alibaba_maas", False, None, 0, 0, "HTTP 404: Not Found", 2),
    ("qwen-flash-character", "alibaba_maas", True, "OK", 2061, 262, None, 2),
    ("qwen-plus-character", "alibaba_maas", True, "OK", 2701, 266, None, 2),
    ("qwen-plus-latest", "alibaba_maas", True, "OK", 4530, 262, None, 2),
    ("qwen-plus-2025-09-11", "alibaba_maas", True, "OK", 1904, 262, None, 2),
    ("qwen-plus-2025-07-14", "alibaba_maas", True, "OK", 5088, None, None, 2),
    ("qwen-plus-2025-04-28", "alibaba_maas", True, "OK", 3279, 266, None, 2),
    ("qwen-max", "alibaba_maas", True, "OK", 4197, 262, None, 2),
    ("qwen-max-2025-09-23", "alibaba_maas", False, None, 0, 0, "HTTP 404: Not Found", 2),
    ("qwen-vl-max", "alibaba_maas", True, "OK", 4997, 262, None, 2),
    ("qwen-vl-plus", "alibaba_maas", True, "OK", 2661, 262, None, 2),
    ("qwen-vl-plus-2025-09-23", "alibaba_maas", False, None, 0, 0, "HTTP 404: Not Found", 2),
]

# Batch 3
batch3_results = [
    ("qwen3-8b", "alibaba_maas", True, "OK", 11035, 820, None, 3),
    ("qwen3-30b-a3b", "alibaba_maas", True, "OK", 12224, 1411, None, 3),
    ("qwen3-235b-a22b-instruct-2507", "alibaba_maas", True, "OK", 3324, 262, None, 3),
    ("qwen3-235b-a22b-thinking-2507", "alibaba_maas", True, "OK", 80670, 4366, None, 3),
    ("qwen2-7b-instruct", "alibaba_maas", False, None, 0, 0, "HTTP 404: Not Found", 3),
    ("qwen-turbo", "alibaba_maas", True, "OK", 2394, 266, None, 3),
    ("qwq-plus", "alibaba_maas", True, "OK", 36569, 1485, None, 3),
    ("qwq-plus-2025-03-05", "alibaba_maas", False, None, 0, 0, "HTTP 400: Bad Request", 3),
    ("qvq-max", "alibaba_maas", False, None, 0, 0, "HTTP 400: Bad Request", 3),
    ("qwen-omni-turbo", "alibaba_maas", True, "OK", 3324, 242, None, 3),
    ("qwen3-omni-flash", "alibaba_maas", True, "OK", 2537, 262, None, 3),
    ("qwen3-omni-flash-realtime", "alibaba_maas", True, "OK", 1165, None, None, 3),
    ("qwen3-vl-235b-a22b-instruct", "alibaba_maas", True, "OK", 6258, 262, None, 3),
    ("qwen3-vl-plus", "alibaba_maas", True, "OK", 4818, 262, None, 3),
    ("qwen3-vl-235b-a22b-thinking", "alibaba_maas", True, "OK", 71367, 3987, None, 3),
    ("qwen-image-3.0-pro", "alibaba_maas", False, None, 0, 0, "HTTP 400: Bad Request", 3),
    ("qwen-image-3.0", "alibaba_maas", False, None, 0, 0, "HTTP 400: Bad Request", 3),
    ("qwen-image-2.0-pro", "alibaba_maas", False, None, 0, 0, "HTTP 400: Bad Request", 3),
    ("qwen-image-2.0", "alibaba_maas", False, None, 0, 0, "HTTP 400: Bad Request", 3),
    ("qwen-image-edit-max", "alibaba_maas", False, None, 0, 0, "HTTP 400: Bad Request", 3),
    ("qwen-image-edit-plus", "alibaba_maas", False, None, 0, 0, "HTTP 400: Bad Request", 3),
    ("text-embedding-v4", "alibaba_maas", False, None, 0, 0, "HTTP 400: Bad Request", 3),
    ("text-embedding-v3", "alibaba_maas", False, None, 0, 0, "HTTP 404: Not Found", 3),
    ("qwen3-asr-flash", "alibaba_maas", False, None, 0, 0, "HTTP 400: Bad Request", 3),
    ("qwen3-tts-flash", "alibaba_maas", False, None, 0, 0, "HTTP 400: Bad Request", 3),
]

test_prompt = "Analyze this Python code and identify the bug: def find_max(numbers): max_val = 0; for n in numbers: if n > max_val: max_val = n; return max_val. What's wrong? Provide the fix."

all_results = batch1_results + batch2_results + batch3_results

for r in all_results:
    cursor.execute("""
        INSERT INTO model_tests (model_id, provider, test_prompt, success, response_text, latency_ms, tokens_used, error_message, test_batch)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    """, (r[0], r[1], test_prompt, r[2], r[3], r[4], r[5], r[6], r[7]))

conn.commit()

# Summary
cursor.execute("SELECT provider, COUNT(*), SUM(CASE WHEN success THEN 1 ELSE 0 END) FROM model_tests GROUP BY provider")
print("\nSummary by provider:")
for row in cursor.fetchall():
    print(f"  {row[0]}: {row[2]}/{row[1]} successful")

cursor.execute("SELECT COUNT(*) FROM model_tests WHERE success=1")
print(f"\nTotal successful: {cursor.fetchone()[0]}")
cursor.execute("SELECT COUNT(*) FROM model_tests WHERE success=0")
print(f"Total failed: {cursor.fetchone()[0]}")

conn.close()
