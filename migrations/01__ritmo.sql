-- Table: _sqlx_migrations
DROP TABLE IF EXISTS _sqlx_migrations;

CREATE TABLE IF NOT EXISTS _sqlx_migrations (
    version        BIGINT    PRIMARY KEY,
    description    TEXT      NOT NULL,
    installed_on   TIMESTAMP NOT NULL
                             DEFAULT CURRENT_TIMESTAMP,
    success        BOOLEAN   NOT NULL,
    checksum       BLOB      NOT NULL,
    execution_time BIGINT    NOT NULL
);


-- Table: aliases
DROP TABLE IF EXISTS aliases;

CREATE TABLE IF NOT EXISTS aliases (
    id        INTEGER NOT NULL,
    name      TEXT    NOT NULL,
    person_id INTEGER NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    ),
    FOREIGN KEY (
        person_id
    )
    REFERENCES people (id) 
);


-- Table: books
DROP TABLE IF EXISTS books;

CREATE TABLE IF NOT EXISTS books (
    id                 INTEGER NOT NULL,
    name               TEXT    NOT NULL,
    publisher_id       INTEGER NULL,
    format_id          INTEGER NULL,
    publication_date   BIGINT,
    acquisition_date   BIGINT,
    last_modified_date BIGINT,
    series_id          INTEGER NULL,
    series_index       INTEGER,
    original_title     TEXT,
    notes              TEXT,
    has_cover          INTEGER,
    has_paper          INTEGER,
    file_link          TEXT    UNIQUE,
    pre_accepted       INTEGER DEFAULT (1),
    PRIMARY KEY (
        id AUTOINCREMENT
    ),
    FOREIGN KEY (
        publisher_id
    )
    REFERENCES publishers (id),
    FOREIGN KEY (
        format_id
    )
    REFERENCES formats (id),
    FOREIGN KEY (
        series_id
    )
    REFERENCES series (series_id) 
);


-- Table: books_contents
DROP TABLE IF EXISTS books_contents;

CREATE TABLE IF NOT EXISTS books_contents (
    book_id    INTEGER NOT NULL,
    content_id INTEGER NOT NULL,
    PRIMARY KEY (
        book_id,
        content_id
    ),
    FOREIGN KEY (
        content_id
    )
    REFERENCES contents (id),
    FOREIGN KEY (
        book_id
    )
    REFERENCES books (id) 
);


-- Table: books_people_roles
DROP TABLE IF EXISTS books_people_roles;

CREATE TABLE IF NOT EXISTS books_people_roles (
    book_id   INTEGER NOT NULL,
    person_id INTEGER NOT NULL,
    role_id   INTEGER NOT NULL,
    PRIMARY KEY (
        book_id,
        person_id,
        role_id
    ),
    FOREIGN KEY (
        person_id
    )
    REFERENCES people (id),
    FOREIGN KEY (
        book_id
    )
    REFERENCES books (id),
    FOREIGN KEY (
        role_id
    )
    REFERENCES roles (id) 
);


-- Table: books_tags
DROP TABLE IF EXISTS books_tags;

CREATE TABLE IF NOT EXISTS books_tags (
    book_id INTEGER NOT NULL,
    tag_id  INTEGER NOT NULL,
    PRIMARY KEY (
        book_id,
        tag_id
    ),
    FOREIGN KEY (
        tag_id
    )
    REFERENCES tags (id),
    FOREIGN KEY (
        book_id
    )
    REFERENCES books (id) 
);


-- Table: contents
DROP TABLE IF EXISTS contents;

CREATE TABLE IF NOT EXISTS contents (
    id               INTEGER NOT NULL,
    name             TEXT    NOT NULL,
    original_title   TEXT,
    publication_date BIGINT,
    notes            TEXT,
    type_id          INTEGER,
    pre_accepted     INTEGER DEFAULT (1),
    PRIMARY KEY (
        id AUTOINCREMENT
    ),
    FOREIGN KEY (
        type_id
    )
    REFERENCES types (id) 
);


-- Table: contents_current_languages
DROP TABLE IF EXISTS contents_current_languages;

CREATE TABLE IF NOT EXISTS contents_current_languages (
    content_id   INTEGER NOT NULL,
    curr_lang_id INTEGER NOT NULL,
    PRIMARY KEY (
        content_id,
        curr_lang_id
    ),
    FOREIGN KEY (
        content_id
    )
    REFERENCES contents (id),
    FOREIGN KEY (
        curr_lang_id
    )
    REFERENCES current_languages (id) 
);


-- Table: contents_original_languages
DROP TABLE IF EXISTS contents_original_languages;

CREATE TABLE IF NOT EXISTS contents_original_languages (
    content_id   INTEGER NOT NULL,
    orig_lang_id INTEGER NOT NULL,
    PRIMARY KEY (
        content_id,
        orig_lang_id
    ),
    FOREIGN KEY (
        orig_lang_id
    )
    REFERENCES original_languages (id),
    FOREIGN KEY (
        content_id
    )
    REFERENCES contents (id) 
);


-- Table: contents_people_roles
DROP TABLE IF EXISTS contents_people_roles;

CREATE TABLE IF NOT EXISTS contents_people_roles (
    content_id INTEGER NOT NULL,
    person_id  INTEGER NOT NULL,
    role_id    INTEGER NOT NULL,
    PRIMARY KEY (
        content_id,
        person_id,
        role_id
    ),
    FOREIGN KEY (
        role_id
    )
    REFERENCES roles (id),
    FOREIGN KEY (
        person_id
    )
    REFERENCES people (id),
    FOREIGN KEY (
        content_id
    )
    REFERENCES contents (id) 
);


-- Table: contents_source_languages
DROP TABLE IF EXISTS contents_source_languages;

CREATE TABLE IF NOT EXISTS contents_source_languages (
    content_id     INTEGER NOT NULL,
    source_lang_id INTEGER NOT NULL,
    PRIMARY KEY (
        content_id,
        source_lang_id
    ),
    FOREIGN KEY (
        content_id
    )
    REFERENCES contents (id),
    FOREIGN KEY (
        source_lang_id
    )
    REFERENCES source_languages (id) 
);


-- Table: contents_tags
DROP TABLE IF EXISTS contents_tags;

CREATE TABLE IF NOT EXISTS contents_tags (
    content_id INTEGER NOT NULL,
    tag_id     INTEGER NOT NULL,
    PRIMARY KEY (
        content_id,
        tag_id
    ),
    FOREIGN KEY (
        content_id
    )
    REFERENCES contents (id),
    FOREIGN KEY (
        tag_id
    )
    REFERENCES tags (id) 
);


-- Table: current_languages
DROP TABLE IF EXISTS current_languages;

CREATE TABLE IF NOT EXISTS current_languages (
    id        INTEGER NOT NULL,
    lang_code TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: formats
DROP TABLE IF EXISTS formats;

CREATE TABLE IF NOT EXISTS formats (
    id   INTEGER NOT NULL,
    name TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: languages_names
DROP TABLE IF EXISTS languages_names;

CREATE TABLE IF NOT EXISTS languages_names (
    id   TEXT NOT NULL,
    name TEXT,
    PRIMARY KEY (
        id
    )
);


-- Table: laverdure
DROP TABLE IF EXISTS laverdure;

CREATE TABLE IF NOT EXISTS laverdure (
    key   TEXT NOT NULL,
    value TEXT,
    PRIMARY KEY (
        key
    )
);


-- Table: original_languages
DROP TABLE IF EXISTS original_languages;

CREATE TABLE IF NOT EXISTS original_languages (
    id        INTEGER NOT NULL,
    lang_code TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: people
DROP TABLE IF EXISTS people;

CREATE TABLE IF NOT EXISTS people (
    id          INTEGER NOT NULL
                        UNIQUE,
    name        TEXT    NOT NULL,
    nationality TEXT,
    birth_date  INTEGER,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: publishers
DROP TABLE IF EXISTS publishers;

CREATE TABLE IF NOT EXISTS publishers (
    id   INTEGER NOT NULL,
    name TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: roles
DROP TABLE IF EXISTS roles;

CREATE TABLE IF NOT EXISTS roles (
    id   INTEGER NOT NULL,
    name TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: series
DROP TABLE IF EXISTS series;

CREATE TABLE IF NOT EXISTS series (
    id   INTEGER NOT NULL,
    name TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: source_languages
DROP TABLE IF EXISTS source_languages;

CREATE TABLE IF NOT EXISTS source_languages (
    id        INTEGER NOT NULL,
    lang_code TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: tags
DROP TABLE IF EXISTS tags;

CREATE TABLE IF NOT EXISTS tags (
    id   INTEGER NOT NULL,
    name TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: types
DROP TABLE IF EXISTS types;

CREATE TABLE IF NOT EXISTS types (
    id   INTEGER NOT NULL,
    name TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Index: idx_books_contents_junction
DROP INDEX IF EXISTS idx_books_contents_junction;

CREATE INDEX IF NOT EXISTS idx_books_contents_junction ON books_contents (
    "book_id",
    "content_id"
);


-- Index: idx_books_core_search
DROP INDEX IF EXISTS idx_books_core_search;

CREATE INDEX IF NOT EXISTS idx_books_core_search ON books (
    "name",
    "series_id",
    "publication_date"
);


-- Index: idx_books_metadata
DROP INDEX IF EXISTS idx_books_metadata;

CREATE INDEX IF NOT EXISTS idx_books_metadata ON books (
    "publisher_id",
    "format_id",
    "series_id"
);


-- Index: idx_books_people_lookup
DROP INDEX IF EXISTS idx_books_people_lookup;

CREATE INDEX IF NOT EXISTS idx_books_people_lookup ON books_people_roles (
    "book_id",
    "person_id"
);


-- Index: idx_books_series_index
DROP INDEX IF EXISTS idx_books_series_index;

CREATE INDEX IF NOT EXISTS idx_books_series_index ON books (
    "series_id",
    "series_index"
);


-- Index: idx_books_tags_lookup
DROP INDEX IF EXISTS idx_books_tags_lookup;

CREATE INDEX IF NOT EXISTS idx_books_tags_lookup ON books_tags (
    "book_id",
    "tag_id"
);


-- Index: idx_books_temporal
DROP INDEX IF EXISTS idx_books_temporal;

CREATE INDEX IF NOT EXISTS idx_books_temporal ON books (
    "publication_date",
    "acquisition_date",
    "last_modified_date"
);


-- Index: idx_contents_core_search
DROP INDEX IF EXISTS idx_contents_core_search;

CREATE INDEX IF NOT EXISTS idx_contents_core_search ON contents (
    "name",
    "type_id",
    "publication_date"
);


-- Index: idx_contents_languages
DROP INDEX IF EXISTS idx_contents_languages;

CREATE INDEX IF NOT EXISTS idx_contents_languages ON contents_current_languages (
    "content_id",
    "curr_lang_id"
);


-- Index: idx_contents_metadata
DROP INDEX IF EXISTS idx_contents_metadata;

CREATE INDEX IF NOT EXISTS idx_contents_metadata ON contents (
    "type_id"
);


-- Index: idx_contents_original_languages
DROP INDEX IF EXISTS idx_contents_original_languages;

CREATE INDEX IF NOT EXISTS idx_contents_original_languages ON contents_original_languages (
    "content_id",
    "orig_lang_id"
);


-- Index: idx_contents_people_lookup
DROP INDEX IF EXISTS idx_contents_people_lookup;

CREATE INDEX IF NOT EXISTS idx_contents_people_lookup ON contents_people_roles (
    "content_id",
    "person_id"
);


-- Index: idx_contents_source_languages
DROP INDEX IF EXISTS idx_contents_source_languages;

CREATE INDEX IF NOT EXISTS idx_contents_source_languages ON contents_source_languages (
    "content_id",
    "source_lang_id"
);


-- Index: idx_contents_tags_lookup
DROP INDEX IF EXISTS idx_contents_tags_lookup;

CREATE INDEX IF NOT EXISTS idx_contents_tags_lookup ON contents_tags (
    "content_id",
    "tag_id"
);


-- Index: idx_contents_temporal
DROP INDEX IF EXISTS idx_contents_temporal;

CREATE INDEX IF NOT EXISTS idx_contents_temporal ON contents (
    "publication_date"
);


-- Index: idx_contents_type_date
DROP INDEX IF EXISTS idx_contents_type_date;

CREATE INDEX IF NOT EXISTS idx_contents_type_date ON contents (
    "type_id",
    "publication_date"
);


-- Index: idx_people_search
DROP INDEX IF EXISTS idx_people_search;

CREATE INDEX IF NOT EXISTS idx_people_search ON people (
    "name",
    "id"
);


-- Index: idx_publishers_search
DROP INDEX IF EXISTS idx_publishers_search;

CREATE INDEX IF NOT EXISTS idx_publishers_search ON publishers (
    "name",
    "id"
);


-- Index: idx_roles_search
DROP INDEX IF EXISTS idx_roles_search;

CREATE INDEX IF NOT EXISTS idx_roles_search ON roles (
    "name"
);


-- Index: idx_series_search
DROP INDEX IF EXISTS idx_series_search;

CREATE INDEX IF NOT EXISTS idx_series_search ON series (
    "name"
);


-- Index: idx_tags_search
DROP INDEX IF EXISTS idx_tags_search;

CREATE INDEX IF NOT EXISTS idx_tags_search ON tags (
    "name"
);


-- Index: idx_v_contents_details
DROP INDEX IF EXISTS idx_v_contents_details;

CREATE INDEX IF NOT EXISTS idx_v_contents_details ON contents (
    "id",
    "type_id",
    "publication_date"
);

PRAGMA foreign_keys = ON;