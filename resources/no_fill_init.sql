PRAGMA foreign_keys = off;
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
    publisher_id       INTEGER,
    format_id          INTEGER,
    publication_date   BIGINT,
    acquisition_date   BIGINT,
    last_modified_date BIGINT,
    series_id          INTEGER,
    series_index       INTEGER,
    original_title     TEXT,
    notes              TEXT,
    has_cover          INTEGER,
    has_paper          INTEGER,
    file_link          TEXT    UNIQUE,
    pre_accepted       INTEGER DEFAULT (1),
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
    REFERENCES series (id),
    PRIMARY KEY (
        id AUTOINCREMENT
    )
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
        book_id
    )
    REFERENCES books (id),
    FOREIGN KEY (
        role_id
    )
    REFERENCES roles (id),
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
    FOREIGN KEY (
        type_id
    )
    REFERENCES types (id),
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);
-- Table: contents_languages
DROP TABLE IF EXISTS contents_languages;
CREATE TABLE IF NOT EXISTS contents_languages (
    contents_id  INTEGER,
    languages_id INTEGER,
    FOREIGN KEY (
        contents_id
    )
    REFERENCES contents (id),
    FOREIGN KEY (
        languages_id
    )
    REFERENCES running_languages (id),
    PRIMARY KEY (
        contents_id,
        languages_id
    )
);
-- Table: contents_people_roles
DROP TABLE IF EXISTS contents_people_roles;
CREATE TABLE IF NOT EXISTS contents_people_roles (
    content_id INTEGER NOT NULL,
    person_id  INTEGER NOT NULL,
    role_id    INTEGER NOT NULL,
    FOREIGN KEY (
        role_id
    )
    REFERENCES roles (id),
    FOREIGN KEY (
        content_id
    )
    REFERENCES contents (id),
    FOREIGN KEY (
        person_id
    )
    REFERENCES people (id),
    PRIMARY KEY (
        content_id,
        person_id,
        role_id
    )
);
-- Table: contents_tags
DROP TABLE IF EXISTS contents_tags;
CREATE TABLE IF NOT EXISTS contents_tags (
    content_id INTEGER NOT NULL,
    tag_id     INTEGER NOT NULL,
    FOREIGN KEY (
        content_id
    )
    REFERENCES contents (id),
    FOREIGN KEY (
        tag_id
    )
    REFERENCES tags (id),
    PRIMARY KEY (
        content_id,
        tag_id
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
    id       INTEGER,
    iso_code TEXT    NOT NULL
                     UNIQUE,
    name     TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);
-- Table: languages_roles
DROP TABLE IF EXISTS languages_roles;
CREATE TABLE IF NOT EXISTS languages_roles (
    id   INTEGER,
    name TEXT    NOT NULL
                 UNIQUE,
    PRIMARY KEY (
        id AUTOINCREMENT
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
-- Table: running_languages
DROP TABLE IF EXISTS running_languages;
CREATE TABLE IF NOT EXISTS running_languages (
    id       INTEGER,
    iso_code TEXT,
    role     INTEGER,
    FOREIGN KEY (
        role
    )
    REFERENCES languages_roles (id),
    FOREIGN KEY (
        iso_code
    )
    REFERENCES languages_names (iso_code),
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
INSERT INTO types ( id,  name  )  VALUES (  2,  'Short novel'  );
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
-- Index: idx_contents_metadata
DROP INDEX IF EXISTS idx_contents_metadata;
CREATE INDEX IF NOT EXISTS idx_contents_metadata ON contents (
    "type_id"
);
-- Index: idx_contents_people_lookup
DROP INDEX IF EXISTS idx_contents_people_lookup;
CREATE INDEX IF NOT EXISTS idx_contents_people_lookup ON contents_people_roles (
    "content_id",
    "person_id"
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
-- View: BooksPeopleRolesDetails
DROP VIEW IF EXISTS BooksPeopleRolesDetails;
CREATE VIEW IF NOT EXISTS BooksPeopleRolesDetails AS
    SELECT bpr.book_id,
           b.name AS book_name,
           p.id AS person_id,
           p.name AS person_name,
           r.name AS role_name
      FROM books_people_roles bpr
           JOIN
           books b ON bpr.book_id = b.id
           JOIN
           people p ON bpr.person_id = p.id
           JOIN
           roles r ON bpr.role_id = r.id;
-- View: BooksTagsDetails
DROP VIEW IF EXISTS BooksTagsDetails;
CREATE VIEW IF NOT EXISTS BooksTagsDetails AS
    SELECT bt.book_id,
           b.name AS book_name,
           t.name AS tag_name
      FROM books_tags bt
           JOIN
           books b ON bt.book_id = b.id
           JOIN
           tags t ON bt.tag_id = t.id;
-- View: BooksWithDetails
DROP VIEW IF EXISTS BooksWithDetails;
CREATE VIEW IF NOT EXISTS BooksWithDetails AS
    SELECT b.id,
           b.name AS book_name,
           p.name AS publisher_name,
           f.name AS format_name,
           s.name AS series_name,
           b.series_index,
           b.publication_date,
           b.acquisition_date,
           b.last_modified_date,
           b.original_title,
           b.notes,
           b.has_cover,
           b.has_paper,
           b.file_link,
           b.pre_accepted
      FROM books b
           LEFT JOIN
           publishers p ON b.publisher_id = p.id
           LEFT JOIN
           formats f ON b.format_id = f.id
           LEFT JOIN
           series s ON b.series_id = s.id;
-- View: ContentsLanguagesDetails
DROP VIEW IF EXISTS ContentsLanguagesDetails;
CREATE VIEW IF NOT EXISTS ContentsLanguagesDetails AS
    SELECT cl.contents_id,
           c.name AS content_name,
           ln.name AS language_name,
           lr.name AS language_role
      FROM contents_languages cl
           JOIN
           contents c ON cl.contents_id = c.id
           JOIN
           running_languages rl ON cl.languages_id = rl.id
           JOIN
           languages_names ln ON rl.iso_code = ln.iso_code
           JOIN
           languages_roles lr ON rl.role = lr.id;
-- View: ContentsPeopleRolesDetails
DROP VIEW IF EXISTS ContentsPeopleRolesDetails;
CREATE VIEW IF NOT EXISTS ContentsPeopleRolesDetails AS
    SELECT cpr.content_id,
           c.name AS content_name,
           p.id AS person_id,
           p.name AS person_name,
           r.name AS role_name
      FROM contents_people_roles cpr
           JOIN
           contents c ON cpr.content_id = c.id
           JOIN
           people p ON cpr.person_id = p.id
           JOIN
           roles r ON cpr.role_id = r.id;
-- View: ContentsTagsDetails
DROP VIEW IF EXISTS ContentsTagsDetails;
CREATE VIEW IF NOT EXISTS ContentsTagsDetails AS
    SELECT ct.content_id,
           c.name AS content_name,
           t.name AS tag_name
      FROM contents_tags ct
           JOIN
           contents c ON ct.content_id = c.id
           JOIN
           tags t ON ct.tag_id = t.id;
-- View: ContentsWithDetails
DROP VIEW IF EXISTS ContentsWithDetails;
CREATE VIEW IF NOT EXISTS ContentsWithDetails AS
    SELECT c.id,
           c.name AS content_name,
           c.original_title,
           c.publication_date,
           c.notes,
           t.name AS type_name,
           c.pre_accepted
      FROM contents c
           LEFT JOIN
           types t ON c.type_id = t.id;
CREATE VIEW ContentsFullDetails AS
SELECT
    c.id AS content_id,
    c.name AS content_name,
    c.original_title,
    c.publication_date,
    c.notes AS content_notes,
    t.name AS type_name,
    c.pre_accepted AS content_pre_accepted,
    p.id AS person_id,
    p.name AS person_name,
    r.name AS role_name,
    tag.name AS tag_name,
    ln.name AS language_name,
    lr.name AS language_role
FROM
    contents c
LEFT JOIN
    types t ON c.type_id = t.id
LEFT JOIN
    contents_people_roles cpr ON c.id = cpr.content_id
LEFT JOIN
    people p ON cpr.person_id = p.id
LEFT JOIN
    roles r ON cpr.role_id = r.id
LEFT JOIN
    contents_tags ct ON c.id = ct.content_id
LEFT JOIN
    tags tag ON ct.tag_id = tag.id
LEFT JOIN
    contents_languages cl ON c.id = cl.contents_id
LEFT JOIN
    running_languages rl ON cl.languages_id = rl.id
LEFT JOIN
    languages_names ln ON rl.iso_code = ln.iso_code
LEFT JOIN
    languages_roles lr ON rl.role = lr.id;
CREATE VIEW BooksFullDetails AS
SELECT
    b.id AS book_id,
    b.name AS book_name,
    p.name AS publisher_name,
    f.name AS format_name,
    b.publication_date,
    b.acquisition_date,
    b.last_modified_date,
    s.name AS series_name,
    b.series_index,
    b.original_title,
    b.notes AS book_notes,
    b.has_cover,
    b.has_paper,
    b.file_link,
    b.pre_accepted AS book_pre_accepted,
    pers.id AS person_id,
    pers.name AS person_name,
    r.name AS role_name,
    tag.name AS tag_name,
    c.id AS content_id,
    c.name AS content_name
FROM
    books b
LEFT JOIN
    publishers p ON b.publisher_id = p.id
LEFT JOIN
    formats f ON b.format_id = f.id
LEFT JOIN
    series s ON b.series_id = s.id
LEFT JOIN
    books_people_roles bpr ON b.id = bpr.book_id
LEFT JOIN
    people pers ON bpr.person_id = pers.id
LEFT JOIN
    roles r ON bpr.role_id = r.id
LEFT JOIN
    books_tags bt ON b.id = bt.book_id
LEFT JOIN
    tags tag ON bt.tag_id = tag.id
LEFT JOIN
    books_contents bc ON b.id = bc.book_id
LEFT JOIN
    contents c ON bc.content_id = c.id;
PRAGMA foreign_keys = on;
