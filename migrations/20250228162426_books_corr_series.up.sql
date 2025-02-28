-- Crea una nuova tabella books_new con la definizione corretta
CREATE TABLE books_new (
    "id"    INTEGER NOT NULL,
    "name"  TEXT NOT NULL,
    "publisher_id"  INTEGER,
    "format_id"     INTEGER,
    "publication_date"      BIGINT,
    "acquisition_date"      BIGINT,
    "last_modified_date"    BIGINT,
    "series_id"     INTEGER,
    "series_index"  INTEGER,
    "original_title"        TEXT,
    "notes" TEXT,
    "has_cover"     INTEGER,
    "has_paper"     INTEGER,
    "file_link"     TEXT UNIQUE,
    "pre_accepted"  INTEGER DEFAULT (1),
    PRIMARY KEY("id" AUTOINCREMENT),
    FOREIGN KEY("format_id") REFERENCES "formats"("id"),
    FOREIGN KEY("series_id") REFERENCES "series"("id"), -- Correzione qui
    FOREIGN KEY("publisher_id") REFERENCES "publishers"("id")
);

-- Elimina la vecchia tabella
DROP TABLE books;

-- Rinomina la nuova tabella con il nome books
ALTER TABLE books_new RENAME TO books;