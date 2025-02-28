PRAGMA foreign_keys=off;

CREATE TABLE temp_original_languages (
    content_id INTEGER NOT NULL,
    language_id TEXT NOT NULL,
    PRIMARY KEY(content_id, language_id)
);
DROP TABLE original_languages;
ALTER TABLE temp_original_languages RENAME TO original_languages;

CREATE TABLE temp_current_languages (
    content_id INTEGER NOT NULL,
    language_id TEXT NOT NULL,
    PRIMARY KEY(content_id, language_id)
);
DROP TABLE current_languages;
ALTER TABLE temp_current_languages RENAME TO current_languages;

CREATE TABLE temp_source_languages (
    content_id INTEGER NOT NULL,
    language_id TEXT NOT NULL,
    PRIMARY KEY(content_id, language_id)
);
DROP TABLE source_languages;
ALTER TABLE temp_source_languages RENAME TO source_languages;

PRAGMA foreign_keys=on;
