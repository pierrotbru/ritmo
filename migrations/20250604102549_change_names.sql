PRAGMA foreign_keys = OFF;
CREATE TABLE people_backup AS SELECT * FROM people;
CREATE TABLE aliases_backup AS SELECT * FROM aliases;
ALTER TABLE people ADD COLUMN given_name TEXT;
ALTER TABLE people ADD COLUMN surname TEXT;
ALTER TABLE people ADD COLUMN middle_names TEXT;
ALTER TABLE people ADD COLUMN title TEXT;
ALTER TABLE people ADD COLUMN suffix TEXT;
ALTER TABLE people ADD COLUMN display_name TEXT;
ALTER TABLE people ADD COLUMN normalized_key TEXT;
ALTER TABLE people ADD COLUMN confidence REAL DEFAULT 1.0;
ALTER TABLE people ADD COLUMN created_at INTEGER;
ALTER TABLE people ADD COLUMN updated_at INTEGER;
ALTER TABLE people ADD COLUMN source TEXT DEFAULT 'biblioteca';
UPDATE people 
SET display_name = name 
WHERE display_name IS NULL;
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
ALTER TABLE aliases ADD COLUMN alias_normalized TEXT;
ALTER TABLE aliases ADD COLUMN confidence REAL DEFAULT 0.9;
ALTER TABLE aliases ADD COLUMN created_at INTEGER;
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
CREATE TABLE IF NOT EXISTS people_phonetic_codes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    person_id INTEGER NOT NULL,
    phonetic_code TEXT NOT NULL,
    code_type TEXT DEFAULT 'metaphone',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_people_normalized_key ON people(normalized_key);
CREATE INDEX IF NOT EXISTS idx_people_given_name ON people(given_name);
CREATE INDEX IF NOT EXISTS idx_people_surname ON people(surname);
CREATE INDEX IF NOT EXISTS idx_people_given_surname ON people(given_name, surname);
CREATE INDEX IF NOT EXISTS idx_people_confidence ON people(confidence DESC);
CREATE INDEX IF NOT EXISTS idx_people_display_name ON people(display_name);
DROP INDEX IF EXISTS idx_people_search; CREATE INDEX IF NOT EXISTS idx_people_search_enhanced ON people(name, normalized_key, id);
CREATE INDEX IF NOT EXISTS idx_aliases_person_id ON aliases(person_id);
CREATE INDEX IF NOT EXISTS idx_aliases_normalized ON aliases(alias_normalized);
CREATE INDEX IF NOT EXISTS idx_aliases_name_enhanced ON aliases(name, person_id);
CREATE INDEX IF NOT EXISTS idx_phonetic_codes_code ON people_phonetic_codes(phonetic_code);
CREATE INDEX IF NOT EXISTS idx_phonetic_codes_person ON people_phonetic_codes(person_id);
CREATE INDEX IF NOT EXISTS idx_phonetic_codes_type ON people_phonetic_codes(code_type, phonetic_code);
CREATE INDEX IF NOT EXISTS idx_books_people_roles_enhanced ON books_people_roles(person_id, role_id, book_id);
CREATE INDEX IF NOT EXISTS idx_contents_people_roles_enhanced ON contents_people_roles(person_id, role_id, content_id);
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
CREATE UNIQUE INDEX IF NOT EXISTS idx_people_normalized_unique 
ON people(normalized_key) 
WHERE normalized_key IS NOT NULL AND normalized_key != '';
CREATE UNIQUE INDEX IF NOT EXISTS idx_phonetic_unique 
ON people_phonetic_codes(person_id, phonetic_code, code_type);
PRAGMA foreign_keys = ON;
