PRAGMA foreign_keys = off;

DROP TABLE IF EXISTS running_languages;
CREATE TABLE IF NOT EXISTS running_languages (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    iso_code TEXT    REFERENCES languages_names (iso_code),
    role     INTEGER REFERENCES languages_roles (id) 
);

DROP TABLE IF EXISTS contents_languages;
CREATE TABLE IF NOT EXISTS contents_languages (
    contents_id  INTEGER REFERENCES contents (id) PRIMARY KEY,
    languages_id INTEGER REFERENCES running_languages (id) 
);

PRAGMA foreign_keys = on;
