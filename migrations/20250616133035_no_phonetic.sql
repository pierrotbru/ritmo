DROP INDEX IF EXISTS idx_people_phonetic_key;
DROP INDEX IF EXISTS idx_phonetic_codes_code;
DROP INDEX IF EXISTS idx_phonetic_codes_person;
DROP INDEX IF EXISTS idx_phonetic_codes_type;
DROP TABLE IF EXISTS people;
CREATE TABLE people (
    id             INTEGER NOT NULL
                           PRIMARY KEY AUTOINCREMENT
                           UNIQUE,
    name           TEXT    NOT NULL,
    nationality    TEXT,
    birth_date     INTEGER,
    given_name     TEXT,
    surname        TEXT,
    middle_names   TEXT,
    title          TEXT,
    suffix         TEXT,
    display_name   TEXT,
    normalized_key TEXT,
    confidence     REAL    DEFAULT 1.0,
    created_at     INTEGER,
    updated_at     INTEGER,
    source         TEXT    DEFAULT 'biblioteca',
    verified       INTEGER
);
DROP TABLE IF EXISTS people_backup;
DROP TABLE IF EXISTS people_phonetic_codes;
