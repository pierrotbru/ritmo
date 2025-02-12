-- people
DROP TABLE IF EXISTS people;
CREATE TABLE IF NOT EXISTS people (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    nationality TEXT,
    birth_year TEXT CHECK (length(birth_year) = 4 AND birth_year GLOB '[0-9]*')
);

-- books
DROP TABLE IF EXISTS books;
CREATE TABLE IF NOT EXISTS books (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    publisher_id INTEGER REFERENCES publishers(id),
    format_id INTEGER REFERENCES formats(id),
    publication_date BIGINT,
    acquisition_date BIGINT,
    last_modified_date BIGINT,
    series_id INTEGER REFERENCES series(series_id),
    series_index INTEGER UNIQUE,
    original_title TEXT,
    notes TEXT
);

-- contents
DROP TABLE IF EXISTS contents;
CREATE TABLE IF NOT EXISTS contents (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    type_id INTEGER REFERENCES contents_types(id),
    issue_date BIGINT,
    original_title TEXT,
    notes TEXT
);

-- contents_types
DROP TABLE IF EXISTS contents_types;
CREATE TABLE IF NOT EXISTS contents_types (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    type_name TEXT NOT NULL UNIQUE
);

-- formats
DROP TABLE IF EXISTS formats;
CREATE TABLE IF NOT EXISTS formats (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    format_name TEXT NOT NULL UNIQUE
);

-- publishers
DROP TABLE IF EXISTS publishers;
CREATE TABLE IF NOT EXISTS publishers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    name TEXT NOT NULL UNIQUE
);

-- roles
DROP TABLE IF EXISTS roles;
CREATE TABLE IF NOT EXISTS roles (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    role_name TEXT NOT NULL UNIQUE
);

-- series
DROP TABLE IF EXISTS series;
CREATE TABLE IF NOT EXISTS series (
    series_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    series TEXT NOT NULL UNIQUE
);

-- tags
DROP TABLE IF EXISTS tags;
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    tag_name TEXT NOT NULL UNIQUE
);

-- languages_names
DROP TABLE IF EXISTS languages_names;
CREATE TABLE IF NOT EXISTS languages_names (
    id TEXT PRIMARY KEY,
    ref_name TEXT
);

-- original_languages (CORRETTA)
DROP TABLE IF EXISTS original_languages;
CREATE TABLE IF NOT EXISTS original_languages (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE, -- Aggiunto id
    lang_code TEXT NOT NULL UNIQUE
);

-- source_languages (CORRETTA)
DROP TABLE IF EXISTS source_languages;
CREATE TABLE IF NOT EXISTS source_languages (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE, -- Aggiunto id
    lang_code TEXT NOT NULL UNIQUE
);

-- current_languages (CORRETTA)
DROP TABLE IF EXISTS current_languages;
CREATE TABLE IF NOT EXISTS current_languages (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE, -- Aggiunto id
    lang_code TEXT NOT NULL UNIQUE
);

-- aliases
DROP TABLE IF EXISTS aliases;
CREATE TABLE IF NOT EXISTS aliases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    alias TEXT NOT NULL UNIQUE, -- L'alias deve essere univoco
    person_id INTEGER NOT NULL,
    FOREIGN KEY (person_id) REFERENCES people(id)
);

-- books_contents
DROP TABLE IF EXISTS books_contents;
CREATE TABLE IF NOT EXISTS books_contents (
    book_id INTEGER NOT NULL REFERENCES books(id),
    content_id INTEGER NOT NULL REFERENCES contents(id),
    PRIMARY KEY (book_id, content_id)
);

-- books_people
DROP TABLE IF EXISTS books_people;
CREATE TABLE IF NOT EXISTS books_people (
    book_id INTEGER NOT NULL REFERENCES books(id),
    person_id INTEGER NOT NULL REFERENCES people(id),
    role_id INTEGER NOT NULL REFERENCES roles(id),
    PRIMARY KEY (book_id, person_id, role_id)
);

-- contents_people
DROP TABLE IF EXISTS contents_people;
CREATE TABLE IF NOT EXISTS contents_people (
    content_id INTEGER NOT NULL REFERENCES contents(id),
    person_id INTEGER NOT NULL REFERENCES people(id),
    role_id INTEGER NOT NULL REFERENCES roles(id),
    PRIMARY KEY (content_id, person_id, role_id)
);

-- contents_tags
DROP TABLE IF EXISTS contents_tags;
CREATE TABLE IF NOT EXISTS contents_tags (
    content_id INTEGER NOT NULL REFERENCES contents(id),
    tag_id INTEGER NOT NULL REFERENCES tags(id),
    PRIMARY KEY (content_id, tag_id)
);

-- books_tags
DROP TABLE IF EXISTS books_tags;
CREATE TABLE IF NOT EXISTS books_tags (
    book_id INTEGER NOT NULL REFERENCES books(id),
    tag_id INTEGER NOT NULL REFERENCES tags(id),
    PRIMARY KEY (book_id, tag_id)
);

-- contents_current_languages (CORRETTA)
DROP TABLE IF EXISTS contents_current_languages;
CREATE TABLE IF NOT EXISTS contents_current_languages (
    content_id INTEGER NOT NULL REFERENCES contents (id),
    curr_lang_id INTEGER NOT NULL REFERENCES current_languages (id),
    PRIMARY KEY (content_id, curr_lang_id)
);

-- contents_original_languages (CORRETTA)
DROP TABLE IF EXISTS contents_original_languages;
CREATE TABLE IF NOT EXISTS contents_original_languages (
    content_id INTEGER NOT NULL REFERENCES contents (id),
    orig_lang_id INTEGER NOT NULL REFERENCES original_languages (id),
    PRIMARY KEY (content_id, orig_lang_id)
);

-- contents_source_languages (CORRETTA)
DROP TABLE IF EXISTS contents_source_languages;
CREATE TABLE IF NOT EXISTS contents_source_languages (
    content_id INTEGER NOT NULL REFERENCES contents (id),
    source_lang_id INTEGER NOT NULL REFERENCES source_languages (id),
    PRIMARY KEY (content_id, source_lang_id)
);

