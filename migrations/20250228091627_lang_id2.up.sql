PRAGMA foreign_keys=off;

CREATE TABLE temp_original_languages (
    id INTEGER NOT NULL,
    iso_code TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
DROP TABLE original_languages;
ALTER TABLE temp_original_languages RENAME TO original_languages;

CREATE TABLE temp_current_languages (
    id INTEGER NOT NULL,
    iso_code TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
DROP TABLE current_languages;
ALTER TABLE temp_current_languages RENAME TO current_languages;

CREATE TABLE temp_source_languages (
    id INTEGER NOT NULL,
    iso_code TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
DROP TABLE source_languages;
ALTER TABLE temp_source_languages RENAME TO source_languages;

PRAGMA foreign_keys=on;