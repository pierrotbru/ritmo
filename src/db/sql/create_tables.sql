-- People
DROP TABLE IF EXISTS People;
CREATE TABLE IF NOT EXISTS People (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    nationality TEXT,
    birth_year TEXT CHECK (length(birth_year) = 4 AND birth_year GLOB '[0-9]*')
);

-- Books
DROP TABLE IF EXISTS Books;
CREATE TABLE IF NOT EXISTS Books (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    publisher_id INTEGER REFERENCES Publishers(id),
    format_id INTEGER REFERENCES Formats(id),
    publication_date BIGINT,
    acquisition_date BIGINT,
    last_modified_date BIGINT,
    series_id INTEGER REFERENCES Series(series_id),
    series_index INTEGER UNIQUE,
    original_title TEXT,
    notes TEXT
);

-- Content
DROP TABLE IF EXISTS Content;
CREATE TABLE IF NOT EXISTS Content (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    type_id INTEGER REFERENCES ContentTypes(id),
    issue_date BIGINT,
    original_title TEXT,
    notes TEXT
);

-- ContentTypes
DROP TABLE IF EXISTS ContentTypes;
CREATE TABLE IF NOT EXISTS ContentTypes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    type_name TEXT NOT NULL UNIQUE
);

-- Formats
DROP TABLE IF EXISTS Formats;
CREATE TABLE IF NOT EXISTS Formats (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    format_name TEXT NOT NULL UNIQUE
);

-- Publishers
DROP TABLE IF EXISTS Publishers;
CREATE TABLE IF NOT EXISTS Publishers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    name TEXT NOT NULL UNIQUE
);

-- Role
DROP TABLE IF EXISTS Role;
CREATE TABLE IF NOT EXISTS Role (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    role_name TEXT NOT NULL UNIQUE
);

-- Series
DROP TABLE IF EXISTS Series;
CREATE TABLE IF NOT EXISTS Series (
    series_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    series TEXT NOT NULL UNIQUE
);

-- Tags
DROP TABLE IF EXISTS Tags;
CREATE TABLE IF NOT EXISTS Tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
    tag_name TEXT NOT NULL UNIQUE
);

-- LangNames
DROP TABLE IF EXISTS LangNames;
CREATE TABLE IF NOT EXISTS LangNames (
    id TEXT PRIMARY KEY,
    ref_name TEXT
);

-- OriginalLanguages (CORRETTA)
DROP TABLE IF EXISTS OriginalLanguages;
CREATE TABLE IF NOT EXISTS OriginalLanguages (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE, -- Aggiunto id
    lang_code TEXT NOT NULL UNIQUE
);

-- SourceLanguages (CORRETTA)
DROP TABLE IF EXISTS SourceLanguages;
CREATE TABLE IF NOT EXISTS SourceLanguages (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE, -- Aggiunto id
    lang_code TEXT NOT NULL UNIQUE
);

-- CurrentLanguages (CORRETTA)
DROP TABLE IF EXISTS CurrentLanguages;
CREATE TABLE IF NOT EXISTS CurrentLanguages (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE, -- Aggiunto id
    lang_code TEXT NOT NULL UNIQUE
);

-- Aliases
DROP TABLE IF EXISTS Aliases;
CREATE TABLE IF NOT EXISTS Aliases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    alias TEXT NOT NULL UNIQUE, -- L'alias deve essere univoco
    person_id INTEGER NOT NULL,
    FOREIGN KEY (person_id) REFERENCES People(id)
);

-- BookContents
DROP TABLE IF EXISTS BookContents;
CREATE TABLE IF NOT EXISTS BookContents (
    book_id INTEGER NOT NULL REFERENCES Books(id),
    content_id INTEGER NOT NULL REFERENCES Content(id),
    PRIMARY KEY (book_id, content_id)
);

-- BookPeople
DROP TABLE IF EXISTS BookPeople;
CREATE TABLE IF NOT EXISTS BookPeople (
    book_id INTEGER NOT NULL REFERENCES Books(id),
    person_id INTEGER NOT NULL REFERENCES People(id),
    role_id INTEGER NOT NULL REFERENCES Role(id),
    PRIMARY KEY (book_id, person_id, role_id)
);

-- ContentPeople
DROP TABLE IF EXISTS ContentPeople;
CREATE TABLE IF NOT EXISTS ContentPeople (
    content_id INTEGER NOT NULL REFERENCES Content(id),
    person_id INTEGER NOT NULL REFERENCES People(id),
    role_id INTEGER NOT NULL REFERENCES Role(id),
    PRIMARY KEY (content_id, person_id, role_id)
);

-- ContentTags
DROP TABLE IF EXISTS ContentTags;
CREATE TABLE IF NOT EXISTS ContentTags (
    content_id INTEGER NOT NULL REFERENCES Content(id),
    tag_id INTEGER NOT NULL REFERENCES Tags(id),
    PRIMARY KEY (content_id, tag_id)
);

-- BooksTags
DROP TABLE IF EXISTS BooksTags;
CREATE TABLE IF NOT EXISTS BooksTags (
    book_id INTEGER NOT NULL REFERENCES Books(id),
    tag_id INTEGER NOT NULL REFERENCES Tags(id),
    PRIMARY KEY (book_id, tag_id)
);

-- ContentCurrLang (CORRETTA)
DROP TABLE IF EXISTS ContentCurrLang;
CREATE TABLE IF NOT EXISTS ContentCurrLang (
    content_id INTEGER NOT NULL REFERENCES Content (id),
    curr_lang_id INTEGER NOT NULL REFERENCES CurrentLanguages (id),
    PRIMARY KEY (content_id, curr_lang_id)
);

-- ContentOrigLang (CORRETTA)
DROP TABLE IF EXISTS ContentOrigLang;
CREATE TABLE IF NOT EXISTS ContentOrigLang (
    content_id INTEGER NOT NULL REFERENCES Content (id),
    orig_lang_id INTEGER NOT NULL REFERENCES OriginalLanguages (id),
    PRIMARY KEY (content_id, orig_lang_id)
);

-- ContentSourcLang (CORRETTA)
DROP TABLE IF EXISTS ContentSourcLang;
CREATE TABLE IF NOT EXISTS ContentSourcLang (
    content_id INTEGER NOT NULL REFERENCES Content (id),
    source_lang_id INTEGER NOT NULL REFERENCES SourceLanguages (id),
    PRIMARY KEY (content_id, source_lang_id)
);

