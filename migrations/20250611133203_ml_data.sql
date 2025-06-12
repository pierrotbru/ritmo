-- Tabella per i dati del modello ML
CREATE TABLE ml_name_data (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    data_type TEXT NOT NULL, -- 'patterns', 'clusters', 'variants'
    data_key TEXT, -- chiave specifica per il tipo di dato
    data_json TEXT NOT NULL,
    version INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Indici per performance
CREATE INDEX idx_ml_data_type ON ml_name_data(data_type);
CREATE INDEX idx_ml_data_type_key ON ml_name_data(data_type, data_key);

-- Trigger per aggiornare updated_at
CREATE TRIGGER update_ml_data_timestamp 
    AFTER UPDATE ON ml_name_data
BEGIN
    UPDATE ml_name_data SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;