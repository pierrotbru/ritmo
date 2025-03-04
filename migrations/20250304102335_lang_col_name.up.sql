PRAGMA foreign_keys=off;

CREATE TABLE "tmp" (
	"content_id"	INTEGER NOT NULL,
	"lang_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","lang_id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id"),
	FOREIGN KEY("lang_id") REFERENCES "current_languages"("id")
);
DROP TABLE contents_current_languages;
ALTER TABLE tmp RENAME TO contents_current_languages;

CREATE TABLE "tmp" (
	"content_id"	INTEGER NOT NULL,
	"lang_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","lang_id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id"),
	FOREIGN KEY("lang_id") REFERENCES "original_languages"("id")
);
DROP TABLE contents_original_languages;
ALTER TABLE tmp RENAME TO contents_original_languages;

CREATE TABLE "tmp" (
	"content_id"	INTEGER NOT NULL,
	"lang_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","lang_id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id"),
	FOREIGN KEY("lang_id") REFERENCES "source_languages"("id")
);
DROP TABLE contents_source_languages;
ALTER TABLE tmp RENAME TO contents_source_languages;

PRAGMA foreign_keys=on;
