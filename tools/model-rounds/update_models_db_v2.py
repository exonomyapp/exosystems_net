import sqlite3

conn = sqlite3.connect("/home/exocrat/models.db")
cursor = conn.cursor()

# Drop and recreate model_registry with correct logic
cursor.execute("DROP TABLE IF EXISTS model_registry")

cursor.execute("""
    CREATE TABLE model_registry (
        model_id TEXT PRIMARY KEY,
        provider TEXT NOT NULL,
        first_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        last_tested TIMESTAMP,
        last_success TIMESTAMP,
        consecutive_failures INTEGER DEFAULT 0,
        is_active BOOLEAN DEFAULT 1,
        total_tests INTEGER DEFAULT 0,
        successful_tests INTEGER DEFAULT 0,
        last_batch INTEGER DEFAULT 0
    )
""")

cursor.execute("CREATE INDEX IF NOT EXISTS idx_model_registry_active ON model_registry(is_active, provider)")

# Populate from model_tests with CORRECT consecutive failure logic
# We need to process batches in order and track consecutive failures
cursor.execute("SELECT DISTINCT test_batch FROM model_tests ORDER BY test_batch")
batches = [row[0] for row in cursor.fetchall()]

# Get all unique models
cursor.execute("SELECT DISTINCT model_id, provider FROM model_tests ORDER BY model_id, provider")
models = cursor.fetchall()

for model_id, provider in models:
    consecutive_failures = 0
    total_tests = 0
    successful_tests = 0
    last_tested = None
    last_success = None
    last_batch = 0
    
    for batch in batches:
        cursor.execute("""
            SELECT success, tested_at FROM model_tests 
            WHERE model_id = ? AND provider = ? AND test_batch = ?
            ORDER BY tested_at DESC LIMIT 1
        """, (model_id, provider, batch))
        result = cursor.fetchone()
        
        if result:
            success, tested_at = result
            total_tests += 1
            last_tested = tested_at
            last_batch = batch
            
            if success:
                successful_tests += 1
                last_success = tested_at
                consecutive_failures = 0  # Reset on success
            else:
                consecutive_failures += 1
    
    is_active = 1 if consecutive_failures < 3 else 0
    
    cursor.execute("""
        INSERT INTO model_registry (model_id, provider, first_seen, last_tested, last_success, 
                                     consecutive_failures, is_active, total_tests, successful_tests, last_batch)
        VALUES (?, ?, datetime('now'), ?, ?, ?, ?, ?, ?, ?)
    """, (model_id, provider, last_tested, last_success, consecutive_failures, is_active, total_tests, successful_tests, last_batch))

conn.commit()

# Show current state
cursor.execute("SELECT provider, COUNT(*), SUM(CASE WHEN is_active THEN 1 ELSE 0 END) FROM model_registry GROUP BY provider")
print("Model Registry by Provider:")
for row in cursor.fetchall():
    print(f"  {row[0]}: {row[2]}/{row[1]} active")

cursor.execute("SELECT model_id, provider, consecutive_failures, is_active FROM model_registry WHERE consecutive_failures >= 3 ORDER BY consecutive_failures DESC")
print("\nModels with 3+ strikes (INACTIVE):")
for row in cursor.fetchall():
    print(f"  {row[0]} ({row[1]}): {row[2]} strikes, active={bool(row[3])}")

cursor.execute("SELECT model_id, provider, consecutive_failures, is_active FROM model_registry WHERE consecutive_failures > 0 AND consecutive_failures < 3 ORDER BY consecutive_failures DESC")
print("\nModels with 1-2 strikes:")
for row in cursor.fetchall():
    print(f"  {row[0]} ({row[1]}): {row[2]} strikes, active={bool(row[3])}")

conn.close()
