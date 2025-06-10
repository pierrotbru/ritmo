-- ==========================================
-- ANALISI OTTIMIZZAZIONI DATABASE SQLITE
-- ==========================================

-- ==========================================
-- 1. OTTIMIZZAZIONI TABELLA PEOPLE
-- ==========================================

-- PROBLEMA: La tabella people è molto basilare per il name matching
-- SOLUZIONE: Aggiungere campi per supportare il matching avanzato

ALTER TABLE people ADD COLUMN given_name TEXT;
ALTER TABLE people ADD COLUMN surname TEXT; 
ALTER TABLE people ADD COLUMN middle_names TEXT;
ALTER TABLE people ADD COLUMN title TEXT;
ALTER TABLE people ADD COLUMN suffix TEXT;
ALTER TABLE people ADD COLUMN display_name TEXT;
ALTER TABLE people ADD COLUMN normalized_key TEXT;
ALTER TABLE people ADD COLUMN confidence REAL DEFAULT 1.0;

-- Aggiornare i dati esistenti (esempio)
UPDATE people SET 
    display_name = name,
    normalized_key = LOWER(TRIM(REPLACE(REPLACE(name, '  ', ' '), '.', '')))
WHERE display_name IS NULL;

-- ==========================================
-- 2. NUOVE TABELLE PER NAME MATCHING
-- ==========================================

-- Tabella per codici fonetici
CREATE TABLE IF NOT EXISTS people_phonetic_codes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id INTEGER NOT NULL,
    phonetic_code TEXT NOT NULL,
    code_type TEXT DEFAULT 'metaphone',
    FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE
);

-- Normalizzare la tabella aliases esistente
ALTER TABLE aliases ADD COLUMN normalized_alias TEXT;
UPDATE aliases SET normalized_alias = LOWER(TRIM(REPLACE(REPLACE(name, '  ', ' '), '.', '')))
WHERE normalized_alias IS NULL;

-- ==========================================
-- 3. INDICI MANCANTI E OTTIMIZZAZIONI
-- ==========================================

-- Indici critici per people (mancanti completamente!)
CREATE INDEX IF NOT EXISTS idx_people_normalized_key ON people(normalized_key);
CREATE INDEX IF NOT EXISTS idx_people_given_name ON people(given_name);
CREATE INDEX IF NOT EXISTS idx_people_surname ON people(surname);
CREATE INDEX IF NOT EXISTS idx_people_given_surname ON people(given_name, surname);
CREATE INDEX IF NOT EXISTS idx_people_confidence ON people(confidence DESC);

-- Indici per aliases (migliorati)
CREATE INDEX IF NOT EXISTS idx_aliases_person_id ON aliases(person_id);
CREATE INDEX IF NOT EXISTS idx_aliases_normalized ON aliases(normalized_alias);
CREATE INDEX IF NOT EXISTS idx_aliases_name ON aliases(name);

-- Indici per phonetic codes
CREATE INDEX IF NOT EXISTS idx_phonetic_codes_code ON people_phonetic_codes(phonetic_code);
CREATE INDEX IF NOT EXISTS idx_phonetic_codes_person ON people_phonetic_codes(person_id);

-- ==========================================
-- 4. PROBLEMI NELLE VIEWS (PERFORMANCE CRITICI!)
-- ==========================================

-- PROBLEMA: Le views esistenti usano GROUP_CONCAT che è molto costoso
-- SOLUZIONE: Views semplificate per casi d'uso specifici

-- View ottimizzata per ricerca base contenuti
DROP VIEW IF EXISTS ContentsSearch;
CREATE VIEW ContentsSearch AS
SELECT 
    c.id,
    c.name,
    c.original_title,
    c.publication_date,
    t.name as type_name,
    c.pre_accepted
FROM contents c
LEFT JOIN types t ON c.type_id = t.id;

-- View ottimizzata per ricerca base libri  
DROP VIEW IF EXISTS BooksSearch;
CREATE VIEW BooksSearch AS
SELECT 
    b.id,
    b.name,
    b.original_title,
    b.publication_date,
    p.name as publisher_name,
    s.name as series_name,
    b.series_index,
    f.name as format_name,
    b.pre_accepted
FROM books b
LEFT JOIN publishers p ON b.publisher_id = p.id
LEFT JOIN series s ON b.series_id = s.id  
LEFT JOIN formats f ON b.format_id = f.id;

-- ==========================================
-- 5. INDICI COMPOSITI MANCANTI
-- ==========================================

-- Per junction tables (molto importanti!)
CREATE INDEX IF NOT EXISTS idx_books_people_roles_composite ON books_people_roles(person_id, role_id, book_id);
CREATE INDEX IF NOT EXISTS idx_contents_people_roles_composite ON contents_people_roles(person_id, role_id, content_id);

-- Per ricerche temporali più efficienti
CREATE INDEX IF NOT EXISTS idx_books_date_series ON books(publication_date, series_id);
CREATE INDEX IF NOT EXISTS idx_contents_date_type ON contents(publication_date, type_id);

-- ==========================================
-- 6. OTTIMIZZAZIONI STRUTTURALI
-- ==========================================

-- Aggiungere constraint di unicità dove mancano
CREATE UNIQUE INDEX IF NOT EXISTS idx_people_normalized_unique ON people(normalized_key) 
WHERE normalized_key IS NOT NULL AND normalized_key != '';

-- Migliorare i constraint esistenti
-- (Nota: alcuni potrebbero richiedere ricreazione tabella in SQLite)

-- ==========================================
-- 7. TABELLE LOOKUP OTTIMIZZATE
-- ==========================================

-- Cache table per ricerche frequenti di people
CREATE TABLE IF NOT EXISTS people_search_cache (
    id INTEGER PRIMARY KEY,
    person_id INTEGER NOT NULL,
    search_key TEXT NOT NULL,
    match_type TEXT NOT NULL, -- 'exact', 'phonetic', 'alias'
    confidence REAL NOT NULL,
    FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_people_cache_search ON people_search_cache(search_key);
CREATE INDEX IF NOT EXISTS idx_people_cache_confidence ON people_search_cache(confidence DESC);

-- ==========================================
-- 8. QUERY OTTIMIZZATE PER NAME MATCHING
-- ==========================================

-- Query per caricare dati name matching (sostituisce le views pesanti)
/*
-- Carica tutti i people con aliases per il matching engine:
SELECT 
    p.id,
    p.name,
    p.display_name,
    p.normalized_key,
    p.given_name,
    p.surname,
    p.middle_names,
    p.title,
    p.suffix,
    p.confidence,
    p.nationality,
    p.birth_date,
    COALESCE(GROUP_CONCAT(DISTINCT a.name), '') as aliases,
    COALESCE(GROUP_CONCAT(DISTINCT a.normalized_alias), '') as normalized_aliases,
    COALESCE(GROUP_CONCAT(DISTINCT pc.phonetic_code), '') as phonetic_codes
FROM people p
LEFT JOIN aliases a ON p.id = a.person_id
LEFT JOIN people_phonetic_codes pc ON p.id = pc.person_id
GROUP BY p.id;
*/

-- ==========================================
-- 9. CONFIGURAZIONI SQLITE RACCOMANDATE
-- ==========================================

-- Attivare WAL mode per migliori performance concurrent
-- PRAGMA journal_mode = WAL;

-- Ottimizzare cache size (regolare secondo disponibilità RAM)
-- PRAGMA cache_size = -64000;  -- 64MB cache

-- Abilitare query planner ottimizzato
-- PRAGMA optimize;

-- Analizzare statistiche per ottimizzazioni automatiche
-- ANALYZE;

-- ==========================================
-- 10. MONITORING E MANUTENZIONE
-- ==========================================

-- Query per monitorare performance indici
/*
SELECT 
    name,
    tbl_name,
    sql
FROM sqlite_master 
WHERE type = 'index' 
    AND name NOT LIKE 'sqlite_%'
ORDER BY tbl_name, name;
*/

-- Query per identificare tabelle senza indici appropriati
/*
SELECT 
    m.name as table_name,
    COUNT(DISTINCT i.name) as index_count
FROM sqlite_master m
LEFT JOIN sqlite_master i ON i.tbl_name = m.name AND i.type = 'index'
WHERE m.type = 'table'
    AND m.name NOT LIKE 'sqlite_%'
GROUP BY m.name
ORDER BY index_count;
*/

-- ==========================================
-- 11. PROBLEMI IDENTIFICATI NELLE VIEWS ESISTENTI
-- ==========================================

/*
PROBLEMI CRITICI nelle views ContentsFullDetails e BooksFullDetails:

1. GROUP_CONCAT su molte tabelle joined causa performance terribili
2. LEFT JOIN multipli senza filtri appropriati  
3. Nessun limite o paginazione
4. GROUP BY senza ORDER BY può dare risultati inconsistenti

RACCOMANDAZIONE: 
- Usare queste views solo per record singoli (WHERE id = ?)
- Per liste, creare query specifiche ottimizzate
- Considerare materializzazione per dati statici
*/

-- View materializzata esempio (se supportata)
-- CREATE MATERIALIZED VIEW people_summary AS 
-- SELECT id, name, normalized_key, confidence FROM people;

-- ==========================================
-- 12. SCRIPT DI MIGRAZIONE SUGGERITO
-- ==========================================

/*
PASSI PER MIGRAZIONE:

1. Backup del database
2. Aggiungere nuove colonne a people
3. Popolare normalized_key e altri campi
4. Creare nuovi indici
5. Creare tabelle phonetic_codes  
6. Testare performance
7. Aggiornare applicazione per usare nuovi campi
8. Rimuovere views pesanti se non necessarie

ATTENZIONE: Testare su copia del database prima!
*/