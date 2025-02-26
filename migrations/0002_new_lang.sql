CREATE TABLE languages_names_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    iso_code TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL
);
DROP TABLE languages_names;
ALTER TABLE languages_names_new RENAME TO languages_names;

CREATE TABLE original_languages_new (
    book_id INTEGER NOT NULL,
    language_id INTEGER NOT NULL,
    PRIMARY KEY (book_id, language_id),
    FOREIGN KEY (language_id) REFERENCES languages_names(id)
);
DROP TABLE original_languages;
ALTER TABLE original_languages_new RENAME TO original_languages;

CREATE TABLE current_languages_new (
    book_id INTEGER NOT NULL,
    language_id INTEGER NOT NULL,
    PRIMARY KEY (book_id, language_id),
    FOREIGN KEY (language_id) REFERENCES languages_names(id)
);
DROP TABLE current_languages;
ALTER TABLE current_languages_new RENAME TO current_languages;

CREATE TABLE source_languages_new (
    book_id INTEGER NOT NULL,
    language_id INTEGER NOT NULL,
    PRIMARY KEY (book_id, language_id),
    FOREIGN KEY (language_id) REFERENCES languages_names(id)
);
DROP TABLE source_languages;
ALTER TABLE source_languages_new RENAME TO source_languages;


