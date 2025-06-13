-- Migrazione per la tabella feedback sui match
CREATE TABLE IF NOT EXISTS ml_feedback (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    feedback_type TEXT NOT NULL, -- "false_positive" | "false_negative"
    name1 TEXT NOT NULL,
    name2 TEXT NOT NULL,
    timestamp INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);