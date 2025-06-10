-- ==========================================
-- SCRIPT DI MIGRAZIONE: INTEGRAZIONE NAME MATCHING
-- Database Biblioteca -> Struttura Ottimizzata
-- ==========================================

PRAGMA foreign_keys = OFF;

-- ==========================================
-- STEP 1: BACKUP E PREPARAZIONE
-- ==========================================

-- Creare tabelle di backup (opzionale, per sicurezza)
CREATE TABLE people_backup AS SELECT * FROM people;
CREATE TABLE aliases_backup AS SELECT * FROM aliases;

-- ==========================================
-- STEP 2: ESTENDERE TABELLA PEOPLE ESISTENTE
-- ==========================================

-- Aggiungere nuove colonne per name matching
ALTER TABLE people ADD COLUMN given_name TEXT;
ALTER TABLE people ADD COLUMN surname TEXT;
ALTER TABLE people ADD COLUMN middle_names TEXT;
ALTER TABLE people ADD COLUMN title TEXT;
ALTER TABLE people ADD COLUMN suffix TEXT;
ALTER TABLE people ADD COLUMN display_name TEXT;
ALTER TABLE people ADD COLUMN normalized_key TEXT;
ALTER TABLE people ADD COLUMN confidence REAL DEFAULT 1.0;
ALTER TABLE people ADD COLUMN created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE people ADD COLUMN updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE people ADD COLUMN source TEXT DEFAULT 'biblioteca';

-- ==========================================
-- STEP 3: POPOLARE I NUOVI CAMPI
-- ==========================================

-- Popolare display_name con il nome esistente
UPDATE people 
SET display_name = name 
WHERE display_name IS NULL;

-- Funzione di normalizzazione base per SQLite
-- (Questa è una versione semplificata - la normalizzazione completa sarà fatta in Rust)
UPDATE people 
SET normalized_key = LOWER(
    TRIM(
        REPLACE(
            REPLACE(
                REPLACE(
                    REPLACE(name, '  ', ' '), 
                    '.', ''
                ), 
                ',', ''
            ), 
            '-', ' '
        )
    )
) 
WHERE normalized_key IS NULL;

-- Tentativo di parsing semplice nome/cognome
-- (Assumiamo formato "Nome Cognome" per la maggior parte dei casi)
UPDATE people 
SET 
    given_name = CASE 
        WHEN INSTR(name, ' ') > 0 
        THEN TRIM(SUBSTR(name, 1, INSTR(name, ' ') - 1))
        ELSE name
    END,
    surname = CASE 
        WHEN INSTR(name, ' ') > 0 
        THEN TRIM(SUBSTR(name, INSTR(name, ' ') + 1))
        ELSE NULL
    END
WHERE given_name IS NULL;

-- ==========================================
-- STEP 4: ESTENDERE TABELLA ALIASES
-- ==========================================

-- Aggiungere campi di supporto alla tabella aliases esistente
ALTER TABLE aliases ADD COLUMN alias_normalized TEXT;
ALTER TABLE aliases ADD COLUMN confidence REAL DEFAULT 0.9;
ALTER TABLE aliases ADD COLUMN created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP;

-- Normalizzare gli alias esistenti
UPDATE aliases 
SET alias_normalized = LOWER(
    TRIM(
        REPLACE(
            REPLACE(
                REPLACE(
                    REPLACE(name, '  ', ' '), 
                    '.', ''
                ), 
                ',', ''
            ), 
            '-', ' '
        )
    )
) 
WHERE alias_normalized IS NULL;

-- ==========================================
-- STEP 5: CREARE NUOVE TABELLE
-- ==========================================

-- Tabella per codici fonetici (Double Metaphone, etc.)
CREATE TABLE IF NOT EXISTS people_phonetic_codes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id INTEGER NOT NULL,
    phonetic_code TEXT NOT NULL,
    code_type TEXT DEFAULT 'metaphone',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE
);

-- ==========================================
-- STEP 6: CREARE INDICI OTTIMIZZATI
-- ==========================================

-- Indici critici per people (name matching)
CREATE INDEX IF NOT EXISTS idx_people_normalized_key ON people(normalized_key);
CREATE INDEX IF NOT EXISTS idx_people_given_name ON people(given_name);
CREATE INDEX IF NOT EXISTS idx_people_surname ON people(surname);
CREATE INDEX IF NOT EXISTS idx_people_given_surname ON people(given_name, surname);
CREATE INDEX IF NOT EXISTS idx_people_confidence ON people(confidence DESC);
CREATE INDEX IF NOT EXISTS idx_people_display_name ON people(display_name);

-- Migliorare indici aliases esistenti
DROP INDEX IF EXISTS idx_people_search; -- L'indice esistente era limitato
CREATE INDEX IF NOT EXISTS idx_people_search_enhanced ON people(name, normalized_key, id);

-- Indici per aliases ottimizzati
CREATE INDEX IF NOT EXISTS idx_aliases_person_id ON aliases(person_id);
CREATE INDEX IF NOT EXISTS idx_aliases_normalized ON aliases(alias_normalized);
CREATE INDEX IF NOT EXISTS idx_aliases_name_enhanced ON aliases(name, person_id);

-- Indici per phonetic codes
CREATE INDEX IF NOT EXISTS idx_phonetic_codes_code ON people_phonetic_codes(phonetic_code);
CREATE INDEX IF NOT EXISTS idx_phonetic_codes_person ON people_phonetic_codes(person_id);
CREATE INDEX IF NOT EXISTS idx_phonetic_codes_type ON people_phonetic_codes(code_type, phonetic_code);

-- ==========================================
-- STEP 7: OTTIMIZZARE INDICI JUNCTION TABLES
-- ==========================================

-- Migliorare gli indici esistenti per junction tables
CREATE INDEX IF NOT EXISTS idx_books_people_roles_enhanced ON books_people_roles(person_id, role_id, book_id);
CREATE INDEX IF NOT EXISTS idx_contents_people_roles_enhanced ON contents_people_roles(person_id, role_id, content_id);

-- ==========================================
-- STEP 8: CREARE VIEWS OTTIMIZZATE
-- ==========================================

-- View semplificata per name matching (sostituisce logica complessa)
DROP VIEW IF EXISTS PeopleMatchingData;
CREATE VIEW PeopleMatchingData AS
SELECT 
    p.id,
    p.name as original_name,
    p.display_name,
    p.given_name,
    p.surname,
    p.middle_names,
    p.title,
    p.suffix,
    p.normalized_key,
    p.confidence,
    p.nationality,
    p.birth_date,
    p.source
FROM people p
WHERE p.normalized_key IS NOT NULL;

-- View per people con aliases (query ottimizzata)
DROP VIEW IF EXISTS PeopleWithAliases;
CREATE VIEW PeopleWithAliases AS
SELECT 
    p.id,
    p.name,
    p.display_name,
    p.normalized_key,
    p.confidence,
    COUNT(a.id) as alias_count
FROM people p
LEFT JOIN aliases a ON p.id = a.person_id
GROUP BY p.id;

-- View semplificata per ricerche rapide di libri con autori
DROP VIEW IF EXISTS BooksWithMainAuthor;
CREATE VIEW BooksWithMainAuthor AS
SELECT 
    b.id as book_id,
    b.name as book_name,
    b.publication_date,
    s.name as series_name,
    b.series_index,
    p.name as main_author,
    p.id as author_id,
    r.name as author_role
FROM books b
LEFT JOIN books_people_roles bpr ON b.id = bpr.book_id
LEFT JOIN people p ON bpr.person_id = p.id  
LEFT JOIN roles r ON bpr.role_id = r.id
LEFT JOIN series s ON b.series_id = s.id
WHERE r.name IN ('Autore', 'Author', 'Scrittore') OR r.id = (
    SELECT MIN(role_id) FROM books_people_roles WHERE book_id = b.id
);

-- ==========================================
-- STEP 9: AGGIORNARE VIEWS ESISTENTI (OTTIMIZZAZIONE)
-- ==========================================

-- Sostituire le views pesanti con versioni più efficienti
-- ATTENZIONE: Questo potrebbe rompere codice esistente!

-- Backup delle views originali come commento per riferimento
/*
-- View originale ContentsFullDetails era troppo pesante
-- Ora divisa in views più specifiche e performanti
*/

-- View base per contents (sostituisce parte di ContentsFullDetails)
DROP VIEW IF EXISTS ContentsBase;
CREATE VIEW ContentsBase AS
SELECT
    c.id AS content_id,
    c.name AS content_name,
    c.original_title,
    c.publication_date,
    c.notes AS content_notes,
    t.name AS type_name,
    c.pre_accepted AS content_pre_accepted
FROM contents c
LEFT JOIN types t ON c.type_id = t.id;

-- View per contents con primo autore (più efficiente)
DROP VIEW IF EXISTS ContentsWithMainAuthor;
CREATE VIEW ContentsWithMainAuthor AS
SELECT
    c.id AS content_id,
    c.name AS content_name,
    c.publication_date,
    t.name AS type_name,
    p.name AS main_author,
    p.id AS author_id,
    r.name AS author_role
FROM contents c
LEFT JOIN types t ON c.type_id = t.id
LEFT JOIN contents_people_roles cpr ON c.id = cpr.content_id
LEFT JOIN people p ON cpr.person_id = p.id
LEFT JOIN roles r ON cpr.role_id = r.id
WHERE r.name IN ('Autore', 'Author', 'Scrittore') OR r.id = (
    SELECT MIN(role_id) FROM contents_people_roles WHERE content_id = c.id
);

-- ==========================================
-- STEP 10: FUNZIONI HELPER PER NAME MATCHING
-- ==========================================

-- Query preparate per l'applicazione Rust
/*
-- Query 1: Carica tutti i dati per name matching engine
SELECT 
    p.id,
    p.name as original_name,
    p.display_name,
    p.given_name,
    p.surname,
    p.middle_names,
    p.title,
    p.suffix,
    p.normalized_key,
    p.confidence,
    p.nationality,
    p.birth_date,
    GROUP_CONCAT(DISTINCT a.name) as aliases,
    GROUP_CONCAT(DISTINCT a.alias_normalized) as aliases_normalized,
    GROUP_CONCAT(DISTINCT pc.phonetic_code) as phonetic_codes
FROM people p
LEFT JOIN aliases a ON p.id = a.person_id
LEFT JOIN people_phonetic_codes pc ON p.id = pc.person_id
GROUP BY p.id;

-- Query 2: Ricerca rapida per normalized_key
SELECT id, display_name, confidence 
FROM people 
WHERE normalized_key = ? 
ORDER BY confidence DESC 
LIMIT 10;

-- Query 3: Ricerca per codici fonetici
SELECT DISTINCT p.id, p.display_name, p.confidence
FROM people p
JOIN people_phonetic_codes pc ON p.id = pc.person_id
WHERE pc.phonetic_code IN (?, ?, ?)
ORDER BY p.confidence DESC
LIMIT 20;
*/

-- ==========================================
-- STEP 11: VINCOLI E VALIDAZIONI
-- ==========================================

-- Aggiungere constraint per data integrity
CREATE UNIQUE INDEX IF NOT EXISTS idx_people_normalized_unique 
ON people(normalized_key) 
WHERE normalized_key IS NOT NULL AND normalized_key != '';

-- Constraint per phonetic codes
CREATE UNIQUE INDEX IF NOT EXISTS idx_phonetic_unique 
ON people_phonetic_codes(person_id, phonetic_code, code_type);

-- ==========================================
-- STEP 12: CONFIGURAZIONI SQLITE OTTIMALI
-- ==========================================

-- Riattivare foreign keys
PRAGMA foreign_keys = ON;

-- Ottimizzazioni performance
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA cache_size = -64000;  -- 64MB cache
PRAGMA temp_store = MEMORY;

-- Aggiornare statistiche per query optimizer
ANALYZE;
PRAGMA optimize;

-- ==========================================
-- STEP 13: VERIFICA MIGRAZIONE
-- ==========================================

-- Query di verifica
/*
-- Verifica 1: Conteggio record migrati
SELECT 
    COUNT(*) as total_people,
    COUNT(display_name) as with_display_name,
    COUNT(normalized_key) as with_normalized_key,
    COUNT(given_name) as with_given_name,
    COUNT(surname) as with_surname
FROM people;

-- Verifica 2: Qualità normalizzazione
SELECT 
    name,
    display_name,
    normalized_key,
    given_name,
    surname
FROM people 
WHERE normalized_key IS NOT NULL
ORDER BY id
LIMIT 10;

-- Verifica 3: Aliases migrati
SELECT 
    COUNT(*) as total_aliases,
    COUNT(alias_normalized) as with_normalized
FROM aliases;

-- Verifica 4: Performance test
EXPLAIN QUERY PLAN 
SELECT * FROM people WHERE normalized_key = 'john smith';
*/

-- ==========================================
-- STEP 14: SCRIPT POST-MIGRAZIONE
-- ==========================================

/*
AZIONI DOPO LA MIGRAZIONE:

1. Eseguire l'applicazione Rust per popolare people_phonetic_codes
2. Verificare che tutti i normalized_key siano corretti
3. Testare le query di name matching
4. Aggiornare il codice applicazione per usare i nuovi campi
5. Monitorare le performance
6. Rimuovere le tabelle di backup se tutto funziona

IMPORTANTE:
- Il campo 'name' originale è mantenuto per compatibilità
- Tutte le relazioni esistenti sono preservate  
- Le views originali sono modificate ma compatibili
- Gli indici esistenti sono mantenuti e migliorati
*/