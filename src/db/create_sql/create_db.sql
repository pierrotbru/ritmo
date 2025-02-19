--
-- File generated with SQLiteStudio v3.4.4 on lun feb 17 17:19:36 2025
--
-- Text encoding used: System
--
PRAGMA foreign_keys = off;
BEGIN TRANSACTION;

-- Table: people
DROP TABLE IF EXISTS people;

CREATE TABLE "people" (
    "id"    INTEGER NOT NULL UNIQUE,
    "name"  TEXT NOT NULL,
    "nationality"   TEXT,
    "birth_date"    INTEGER,
    "role"  INTEGER NOT NULL,
    FOREIGN KEY("role") REFERENCES "roles"("id"),
    PRIMARY KEY("id" AUTOINCREMENT)
);

-- Table: aliases
DROP TABLE IF EXISTS aliases;

CREATE TABLE IF NOT EXISTS aliases (
    id        INTEGER NOT NULL
                      PRIMARY KEY AUTOINCREMENT,
    alias     TEXT    NOT NULL,
    person_id INTEGER NOT NULL,
    FOREIGN KEY (
        person_id
    )
    REFERENCES people (id) 
);


-- Table: books
DROP TABLE IF EXISTS books;

CREATE TABLE IF NOT EXISTS books (
    id                 INTEGER NOT NULL
                               PRIMARY KEY AUTOINCREMENT,
    title              TEXT    NOT NULL,
    publisher_id       INTEGER,
    format_id          INTEGER,
    publication_date   BIGINT,
    acquisition_date   BIGINT,
    last_modified_date BIGINT,
    series_id          INTEGER,
    series_index       INTEGER,
    original_title     TEXT,
    notes              TEXT,
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
        book_id
    )
    REFERENCES books (id),
    FOREIGN KEY (
        content_id
    )
    REFERENCES contents (id) 
);


-- Table: books_people
DROP TABLE IF EXISTS books_people;

CREATE TABLE IF NOT EXISTS books_people (
    book_id   INTEGER NOT NULL,
    person_id INTEGER NOT NULL,
    PRIMARY KEY (
        book_id,
        person_id
    ),
    FOREIGN KEY (
        book_id
    )
    REFERENCES books (id),
    FOREIGN KEY (
        person_id
    )
    REFERENCES people (id) 
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
        book_id
    )
    REFERENCES books (id),
    FOREIGN KEY (
        tag_id
    )
    REFERENCES tags (id) 
);


-- Table: contents
DROP TABLE IF EXISTS contents;

CREATE TABLE IF NOT EXISTS contents (
    id               INTEGER NOT NULL
                             PRIMARY KEY AUTOINCREMENT,
    title            TEXT    NOT NULL,
    original_title   TEXT,
    publication_date BIGINT,
    notes            TEXT,
    type_id          INTEGER,
    FOREIGN KEY (
        type_id
    )
    REFERENCES contents_types (id) 
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
        content_id
    )
    REFERENCES contents (id),
    FOREIGN KEY (
        orig_lang_id
    )
    REFERENCES original_languages (id) 
);


-- Table: contents_people
DROP TABLE IF EXISTS contents_people;

CREATE TABLE IF NOT EXISTS contents_people (
    content_id INTEGER NOT NULL,
    person_id  INTEGER NOT NULL,
    PRIMARY KEY (
        content_id,
        person_id
    ),
    FOREIGN KEY (
        content_id
    )
    REFERENCES contents (id),
    FOREIGN KEY (
        person_id
    )
    REFERENCES people (id) 
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


-- Table: contents_types
DROP TABLE IF EXISTS contents_types;

CREATE TABLE IF NOT EXISTS contents_types (
    id        INTEGER NOT NULL
                      PRIMARY KEY AUTOINCREMENT,
    type_name TEXT    NOT NULL
);


-- Table: current_languages
DROP TABLE IF EXISTS current_languages;

CREATE TABLE IF NOT EXISTS current_languages (
    id        INTEGER NOT NULL
                      PRIMARY KEY AUTOINCREMENT,
    lang_code TEXT    NOT NULL
);


-- Table: formats
DROP TABLE IF EXISTS formats;

CREATE TABLE IF NOT EXISTS formats (
    id          INTEGER NOT NULL
                        PRIMARY KEY AUTOINCREMENT,
    format_name TEXT    NOT NULL
);


-- Table: languages_names
DROP TABLE IF EXISTS languages_names;

CREATE TABLE IF NOT EXISTS languages_names (
    id       TEXT NOT NULL
                  PRIMARY KEY,
    ref_name TEXT
);


-- Table: laverdure
DROP TABLE IF EXISTS laverdure;

CREATE TABLE IF NOT EXISTS laverdure (
    key   TEXT NOT NULL
               PRIMARY KEY,
    value TEXT
);


-- Table: original_languages
DROP TABLE IF EXISTS original_languages;

CREATE TABLE IF NOT EXISTS original_languages (
    id        INTEGER NOT NULL
                      PRIMARY KEY AUTOINCREMENT,
    lang_code TEXT    NOT NULL
);




-- Table: publishers
DROP TABLE IF EXISTS publishers;

CREATE TABLE IF NOT EXISTS publishers (
    id   INTEGER NOT NULL
                 PRIMARY KEY AUTOINCREMENT,
    name TEXT    NOT NULL
);


-- Table: roles
DROP TABLE IF EXISTS roles;

CREATE TABLE IF NOT EXISTS roles (
    id   INTEGER NOT NULL
                 PRIMARY KEY AUTOINCREMENT,
    name TEXT    NOT NULL
);


-- Table: series
DROP TABLE IF EXISTS series;

CREATE TABLE IF NOT EXISTS series (
    series_id INTEGER NOT NULL
                      PRIMARY KEY AUTOINCREMENT,
    series    TEXT    NOT NULL
);


-- Table: source_languages
DROP TABLE IF EXISTS source_languages;

CREATE TABLE IF NOT EXISTS source_languages (
    id        INTEGER NOT NULL
                      PRIMARY KEY AUTOINCREMENT,
    lang_code TEXT    NOT NULL
);


-- Table: tags
DROP TABLE IF EXISTS tags;

CREATE TABLE IF NOT EXISTS tags (
    id       INTEGER NOT NULL
                     PRIMARY KEY AUTOINCREMENT,
    tag_name TEXT    NOT NULL
);


-- Index: idx_books_contents_junction
DROP INDEX IF EXISTS idx_books_contents_junction;

CREATE INDEX IF NOT EXISTS idx_books_contents_junction ON books_contents (
    book_id,
    content_id
);


-- Index: idx_books_core_search
DROP INDEX IF EXISTS idx_books_core_search;

CREATE INDEX IF NOT EXISTS idx_books_core_search ON books (
    title,
    series_id,
    publication_date
);


-- Index: idx_books_metadata
DROP INDEX IF EXISTS idx_books_metadata;

CREATE INDEX IF NOT EXISTS idx_books_metadata ON books (
    publisher_id,
    format_id,
    series_id
);


-- Index: idx_books_people_lookup
DROP INDEX IF EXISTS idx_books_people_lookup;

CREATE INDEX IF NOT EXISTS idx_books_people_lookup ON books_people (
    book_id,
    person_id
);


-- Index: idx_books_series_index
DROP INDEX IF EXISTS idx_books_series_index;

CREATE INDEX IF NOT EXISTS idx_books_series_index ON books (
    series_id,
    series_index
);


-- Index: idx_books_tags_lookup
DROP INDEX IF EXISTS idx_books_tags_lookup;

CREATE INDEX IF NOT EXISTS idx_books_tags_lookup ON books_tags (
    book_id,
    tag_id
);


-- Index: idx_books_temporal
DROP INDEX IF EXISTS idx_books_temporal;

CREATE INDEX IF NOT EXISTS idx_books_temporal ON books (
    publication_date,
    acquisition_date,
    last_modified_date
);


-- Index: idx_contents_core_search
DROP INDEX IF EXISTS idx_contents_core_search;

CREATE INDEX IF NOT EXISTS idx_contents_core_search ON contents (
    title,
    type_id,
    publication_date
);


-- Index: idx_contents_languages
DROP INDEX IF EXISTS idx_contents_languages;

CREATE INDEX IF NOT EXISTS idx_contents_languages ON contents_current_languages (
    content_id,
    curr_lang_id
);


-- Index: idx_contents_metadata
DROP INDEX IF EXISTS idx_contents_metadata;

CREATE INDEX IF NOT EXISTS idx_contents_metadata ON contents (
    type_id
);


-- Index: idx_contents_original_languages
DROP INDEX IF EXISTS idx_contents_original_languages;

CREATE INDEX IF NOT EXISTS idx_contents_original_languages ON contents_original_languages (
    content_id,
    orig_lang_id
);


-- Index: idx_contents_people_lookup
DROP INDEX IF EXISTS idx_contents_people_lookup;

CREATE INDEX IF NOT EXISTS idx_contents_people_lookup ON contents_people (
    content_id,
    person_id
);


-- Index: idx_contents_source_languages
DROP INDEX IF EXISTS idx_contents_source_languages;

CREATE INDEX IF NOT EXISTS idx_contents_source_languages ON contents_source_languages (
    content_id,
    source_lang_id
);


-- Index: idx_contents_tags_lookup
DROP INDEX IF EXISTS idx_contents_tags_lookup;

CREATE INDEX IF NOT EXISTS idx_contents_tags_lookup ON contents_tags (
    content_id,
    tag_id
);


-- Index: idx_contents_temporal
DROP INDEX IF EXISTS idx_contents_temporal;

CREATE INDEX IF NOT EXISTS idx_contents_temporal ON contents (
    publication_date
);


-- Index: idx_contents_type_date
DROP INDEX IF EXISTS idx_contents_type_date;

CREATE INDEX IF NOT EXISTS idx_contents_type_date ON contents (
    type_id,
    publication_date
);


-- Index: idx_people_search
DROP INDEX IF EXISTS idx_people_search;

CREATE INDEX IF NOT EXISTS idx_people_search ON people (
    name,
    id
);


-- Index: idx_publishers_search
DROP INDEX IF EXISTS idx_publishers_search;

CREATE INDEX IF NOT EXISTS idx_publishers_search ON publishers (
    name,
    id
);


-- Index: idx_roles_search
DROP INDEX IF EXISTS idx_roles_search;

CREATE INDEX IF NOT EXISTS idx_roles_search ON roles (
    name
);


-- Index: idx_series_search
DROP INDEX IF EXISTS idx_series_search;

CREATE INDEX IF NOT EXISTS idx_series_search ON series (
    series
);


-- Index: idx_tags_search
DROP INDEX IF EXISTS idx_tags_search;

CREATE INDEX IF NOT EXISTS idx_tags_search ON tags (
    tag_name
);


-- Index: idx_v_contents_details
DROP INDEX IF EXISTS idx_v_contents_details;

CREATE INDEX IF NOT EXISTS idx_v_contents_details ON contents (
    id,
    type_id,
    publication_date
);


-- View: v_full_books
DROP VIEW IF EXISTS v_full_books;
CREATE VIEW IF NOT EXISTS v_full_books AS
    SELECT b.id AS book_id,
           b.title AS book_title,
           b.original_title AS book_original_title,
           b.publication_date,
           b.acquisition_date,
           b.last_modified_date,
           b.notes AS book_notes,
           p.name AS publisher_name,
           f.format_name,
           s.series AS series_name,
           s.series_id,
           b.series_index,
           (SELECT GROUP_CONCAT(DISTINCT tag_name) 
            FROM books_tags bt 
            JOIN tags t ON bt.tag_id = t.id 
            WHERE bt.book_id = b.id) AS book_tags,
           (SELECT GROUP_CONCAT(DISTINCT pe.name || ' (' || COALESCE(r.name, 'Unknown Role') || ')') 
            FROM books_people bp 
            JOIN people pe ON bp.person_id = pe.id 
            LEFT JOIN roles r ON pe.role = r.id 
            WHERE bp.book_id = b.id) AS book_people,
           (SELECT GROUP_CONCAT(DISTINCT c.title) 
            FROM books_contents bc 
            JOIN contents c ON bc.content_id = c.id 
            WHERE bc.book_id = b.id) AS content_titles,
           (SELECT GROUP_CONCAT(DISTINCT c.original_title) 
            FROM books_contents bc 
            JOIN contents c ON bc.content_id = c.id 
            WHERE bc.book_id = b.id) AS content_original_titles,
           (SELECT GROUP_CONCAT(DISTINCT c.publication_date) 
            FROM books_contents bc 
            JOIN contents c ON bc.content_id = c.id 
            WHERE bc.book_id = b.id) AS content_publication_dates,
           (SELECT GROUP_CONCAT(DISTINCT c.notes) 
            FROM books_contents bc 
            JOIN contents c ON bc.content_id = c.id 
            WHERE bc.book_id = b.id) AS content_notes,
           (SELECT GROUP_CONCAT(DISTINCT ct.type_name) 
            FROM books_contents bc 
            JOIN contents c ON bc.content_id = c.id 
            JOIN contents_types ct ON c.type_id = ct.id 
            WHERE bc.book_id = b.id) AS content_types,
           (SELECT GROUP_CONCAT(DISTINCT cl.lang_code) 
            FROM books_contents bc 
            JOIN contents c ON bc.content_id = c.id 
            JOIN contents_current_languages ccl ON c.id = ccl.content_id 
            JOIN current_languages cl ON ccl.curr_lang_id = cl.id 
            WHERE bc.book_id = b.id) AS content_current_languages,
           (SELECT GROUP_CONCAT(DISTINCT ol.lang_code) 
            FROM books_contents bc 
            JOIN contents c ON bc.content_id = c.id 
            JOIN contents_original_languages col ON c.id = col.content_id 
            JOIN original_languages ol ON col.orig_lang_id = ol.id 
            WHERE bc.book_id = b.id) AS content_original_languages,
           (SELECT GROUP_CONCAT(DISTINCT sl.lang_code) 
            FROM books_contents bc 
            JOIN contents c ON bc.content_id = c.id 
            JOIN contents_source_languages csl ON c.id = csl.content_id 
            JOIN source_languages sl ON csl.source_lang_id = sl.id 
            WHERE bc.book_id = b.id) AS content_source_languages
      FROM books AS b
           LEFT JOIN publishers AS p ON b.publisher_id = p.id
           LEFT JOIN formats AS f ON b.format_id = f.id
           LEFT JOIN series AS s ON b.series_id = s.series_id;

-- View: v_full_contents
DROP VIEW IF EXISTS v_full_contents;
CREATE VIEW IF NOT EXISTS v_full_contents AS
    SELECT c.id AS content_id,
           c.title AS content_title,
           c.original_title AS content_original_title,
           c.publication_date AS issue_date,
           c.notes AS content_notes,
           ct.type_name,
           (SELECT GROUP_CONCAT(DISTINCT t.tag_name) 
            FROM contents_tags ctg 
            JOIN tags t ON ctg.tag_id = t.id 
            WHERE ctg.content_id = c.id) AS content_tags,
           (SELECT GROUP_CONCAT(DISTINCT pe.name || ' (' || COALESCE(r.name, 'Unknown Role') || ')') 
            FROM contents_people cp 
            JOIN people pe ON cp.person_id = pe.id 
            LEFT JOIN roles r ON pe.role = r.id 
            WHERE cp.content_id = c.id) AS content_people,
           (SELECT GROUP_CONCAT(DISTINCT cl.lang_code) 
            FROM contents_current_languages ccl 
            JOIN current_languages cl ON ccl.curr_lang_id = cl.id 
            WHERE ccl.content_id = c.id) AS content_current_languages,
           (SELECT GROUP_CONCAT(DISTINCT ol.lang_code) 
            FROM contents_original_languages col 
            JOIN original_languages ol ON col.orig_lang_id = ol.id 
            WHERE col.content_id = c.id) AS content_original_languages,
           (SELECT GROUP_CONCAT(DISTINCT sl.lang_code) 
            FROM contents_source_languages csl 
            JOIN source_languages sl ON csl.source_lang_id = sl.id 
            WHERE csl.content_id = c.id) AS content_source_languages,
           b.id AS book_id,
           b.title AS book_title,
           b.original_title AS book_original_title
      FROM contents AS c
           LEFT JOIN contents_types AS ct ON c.type_id = ct.id
           LEFT JOIN books_contents AS bc ON c.id = bc.content_id
           LEFT JOIN books AS b ON bc.book_id = b.id;

COMMIT TRANSACTION;
PRAGMA foreign_keys = on;
