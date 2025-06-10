-- Tabella principale delle persone
CREATE TABLE persons (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    
    -- Nome originale completo come inserito
    full_name TEXT NOT NULL,
    
    -- Componenti del nome parsato
    given_name TEXT,
    surname TEXT,
    middle_names TEXT, -- JSON array o stringa separata da virgole
    title TEXT,
    suffix TEXT,
    
    -- Nome per display (utilizzato nella funzione)
    display_name TEXT NOT NULL,
    
    -- Chiave normalizzata per matching diretto (molto importante!)
    normalized_key TEXT NOT NULL,
    
    -- Confidence score del record
    confidence REAL DEFAULT 1.0,
    
    -- Metadati
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    source TEXT, -- da dove proviene il dato
    
    -- Constraints
    CONSTRAINT chk_confidence CHECK (confidence >= 0.0 AND confidence <= 1.0)
);

-- Tabella per gli alias (relazione 1:N)
CREATE TABLE person_aliases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id INTEGER NOT NULL,
    alias_name TEXT NOT NULL,
    alias_normalized TEXT NOT NULL, -- Versione normalizzata dell'alias
    confidence REAL DEFAULT 0.9, -- Gli alias hanno confidence leggermente inferiore
    
    FOREIGN KEY (person_id) REFERENCES persons(id) ON DELETE CASCADE,
    CONSTRAINT chk_alias_confidence CHECK (confidence >= 0.0 AND confidence <= 1.0)
);

-- Tabella per i codici fonetici (Double Metaphone)
CREATE TABLE phonetic_codes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id INTEGER NOT NULL,
    phonetic_code TEXT NOT NULL,
    code_type TEXT DEFAULT 'metaphone', -- per supportare altri algoritmi in futuro
    
    FOREIGN KEY (person_id) REFERENCES persons(id) ON DELETE CASCADE
);

-- INDICI CRITICI per le performance di matching

-- Indice principale per normalized_key (usato nel Test 1)
CREATE INDEX idx_persons_normalized_key ON persons(normalized_key);

-- Indice per ricerca fonetica
CREATE INDEX idx_phonetic_codes_code ON phonetic_codes(phonetic_code);

-- Indice per alias normalizzati
CREATE INDEX idx_aliases_normalized ON person_aliases(alias_normalized);

-- Indici per ricerche sui componenti del nome
CREATE INDEX idx_persons_given_name ON persons(given_name);
CREATE INDEX idx_persons_surname ON persons(surname);

-- Indice composto per ricerche nome+cognome
CREATE INDEX idx_persons_given_surname ON persons(given_name, surname);

-- Indice per confidence (per ordinamento finale)
CREATE INDEX idx_persons_confidence ON persons(confidence DESC);

-- Indice per full-text search (opzionale, per ricerche testuali avanzate)
CREATE VIRTUAL TABLE persons_fts USING fts5(
    full_name, 
    display_name, 
    given_name, 
    surname,
    content='persons',
    content_rowid='id'
);

-- Trigger per mantenere la tabella FTS aggiornata
CREATE TRIGGER persons_fts_insert AFTER INSERT ON persons BEGIN
    INSERT INTO persons_fts(rowid, full_name, display_name, given_name, surname)
    VALUES (new.id, new.full_name, new.display_name, new.given_name, new.surname);
END;

CREATE TRIGGER persons_fts_delete AFTER DELETE ON persons BEGIN
    DELETE FROM persons_fts WHERE rowid = old.id;
END;

CREATE TRIGGER persons_fts_update AFTER UPDATE ON persons BEGIN
    DELETE FROM persons_fts WHERE rowid = old.id;
    INSERT INTO persons_fts(rowid, full_name, display_name, given_name, surname)
    VALUES (new.id, new.full_name, new.display_name, new.given_name, new.surname);
END;

-- VIEW per facilitare le query di matching
CREATE VIEW person_matching_data AS
SELECT 
    p.id,
    p.full_name,
    p.display_name,
    p.given_name,
    p.surname,
    p.middle_names,
    p.title,
    p.suffix,
    p.normalized_key,
    p.confidence,
    GROUP_CONCAT(DISTINCT pa.alias_name) as aliases,
    GROUP_CONCAT(DISTINCT pc.phonetic_code) as phonetic_codes
FROM persons p
LEFT JOIN person_aliases pa ON p.id = pa.person_id
LEFT JOIN phonetic_codes pc ON p.id = pc.person_id
GROUP BY p.id;

-- Esempio di query per popolare gli indici in-memory della tua applicazione Rust:

-- Query per costruire normalized_key_index
-- SELECT normalized_key, GROUP_CONCAT(id) as person_ids FROM persons GROUP BY normalized_key;

-- Query per costruire phonetic_index  
-- SELECT phonetic_code, GROUP_CONCAT(person_id) as person_ids FROM phonetic_codes GROUP BY phonetic_code;

-- Query per caricare tutti i record con alias
-- SELECT * FROM person_matching_data;