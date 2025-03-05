PRAGMA foreign_keys = off;
DROP TABLE IF EXISTS languages_roles;
CREATE TABLE IF NOT EXISTS languages_roles ( id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT UNIQUE NOT NULL );
INSERT INTO languages_roles ( id, name ) VALUES ( 1, 'source' );
INSERT INTO languages_roles ( id, name ) VALUES ( 2, 'original' );
INSERT INTO languages_roles ( id, name ) VALUES ( 3, 'current' );

DROP TABLE IF EXISTS running_languages;
CREATE TABLE IF NOT EXISTS contents_languages (
    id       INTEGER PRIMARY KEY AUTOINCREMENT,
    iso_code TEXT    REFERENCES languages_names (iso_code),
    role     INTEGER REFERENCES languages_roles (id) 
);

DROP TABLE IF EXISTS contents_languages;
CREATE TABLE IF NOT EXISTS contents_languages (
    contents_id  INTEGER REFERENCES contents (id) PRIMARY KEY,
    languages_id INTEGER REFERENCES running_languages (id) 
);

DROP TABLE contents_source_languages;
DROP TABLE contents_current_languages;
DROP TABLE contents_original_languages;

PRAGMA foreign_keys = on;
