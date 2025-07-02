CREATE TABLE IF NOT EXISTS ml_publisher_feedback (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    feedback_type TEXT NOT NULL,
    publisher1 TEXT NOT NULL,
    publisher2 TEXT NOT NULL,
    timestamp TEXT NOT NULL
);