-- Tabella unica per clusters, patterns, frequenze (per tutti i tipi)
CREATE TABLE IF NOT EXISTS ml_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    data_type TEXT NOT NULL UNIQUE,
    data_json TEXT NOT NULL
);