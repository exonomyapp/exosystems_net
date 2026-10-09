import sqlite3

conn = sqlite3.connect("/home/exocrat/models.db")
cursor = conn.cursor()

# Add new columns to model_tests table for tracking strikes
cursor.execute("""
    ALTER TABLE model_tests ADD COLUMN consecutive_failures INTEGER DEFAULT 0
""")

cursor.execute("""
    ALTER TABLE model_tests ADD COLUMN is_new_model BOOLEAN DEFAULT 0
""")

# Create a models summary table for active status tracking
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
        successful_tests INTEGER DEFAULT 0
    )
""")

# Create index
cursor.execute("CREATE INDEX IF NOT EXISTS idx_model_registry_active ON model_registry(is_active, provider)")

# Populate model_registry from existing model_tests
cursor.execute("""
    INSERT OR IGNORE INTO model_registry (model_id, provider, last_tested, consecutive_failures, is_active, total_tests, successful_tests)
    SELECT 
        model_id,
        provider,
        MAX(tested_at),
        CASE 
            WHEN MAX(CASE WHEN success THEN 0 ELSE 1 END) = 1 THEN 1 ELSE 0
        END,
        CASE 
            WHEN MAX(CASE WHEN success THEN 1 ELSE 0 END) = 1 THEN 1 ELSE 0
        END,
        COUNT(*),
        SUM(CASE WHEN success THEN 1 ELSE 0 END)
    FROM model_tests
    GROUP BY model_id, provider
""")

conn.commit()

# Show current state
cursor.execute("SELECT provider, COUNT(*), SUM(CASE WHEN is_active THEN 1 ELSE 0 END) FROM model_registry GROUP BY provider")
print("Model Registry by Provider:")
for row in cursor.fetchall():
    print(f"  {row[0]}: {row[2]}/{row[1]} active")

cursor.execute("SELECT model_id, provider, consecutive_failures, is_active FROM model_registry ORDER BY consecutive_failures DESC, provider")
print("\nModels with failures:")
for row in cursor.fetchall():
    if row[2] > 0:
        print(f"  {row[0]} ({row[1]}): {row[2]} strikes, active={bool(row[3])}")

conn.close()
