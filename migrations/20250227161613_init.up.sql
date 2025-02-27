-- Add up migration script here
CREATE TABLE IF NOT EXISTS "aliases" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"person_id"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("person_id") REFERENCES "people"("id")
);
CREATE TABLE IF NOT EXISTS "books" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"publisher_id"	INTEGER,
	"format_id"	INTEGER,
	"publication_date"	BIGINT,
	"acquisition_date"	BIGINT,
	"last_modified_date"	BIGINT,
	"series_id"	INTEGER,
	"series_index"	INTEGER,
	"original_title"	TEXT,
	"notes"	TEXT,
	"has_cover"	INTEGER,
	"has_paper"	INTEGER,
	"file_link"	TEXT UNIQUE,
	"pre_accepted"	INTEGER DEFAULT (1),
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("series_id") REFERENCES "series"("series_id"),
	FOREIGN KEY("format_id") REFERENCES "formats"("id"),
	FOREIGN KEY("publisher_id") REFERENCES "publishers"("id")
);
CREATE TABLE IF NOT EXISTS "books_contents" (
	"book_id"	INTEGER NOT NULL,
	"content_id"	INTEGER NOT NULL,
	PRIMARY KEY("book_id","content_id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id"),
	FOREIGN KEY("book_id") REFERENCES "books"("id")
);
CREATE TABLE IF NOT EXISTS "books_people_roles" (
	"book_id"	INTEGER NOT NULL,
	"person_id"	INTEGER NOT NULL,
	"role_id"	INTEGER NOT NULL,
	PRIMARY KEY("book_id","person_id","role_id"),
	FOREIGN KEY("person_id") REFERENCES "people"("id"),
	FOREIGN KEY("book_id") REFERENCES "books"("id"),
	FOREIGN KEY("role_id") REFERENCES "roles"("id")
);
CREATE TABLE IF NOT EXISTS "books_tags" (
	"book_id"	INTEGER NOT NULL,
	"tag_id"	INTEGER NOT NULL,
	PRIMARY KEY("book_id","tag_id"),
	FOREIGN KEY("tag_id") REFERENCES "tags"("id"),
	FOREIGN KEY("book_id") REFERENCES "books"("id")
);
CREATE TABLE IF NOT EXISTS "contents" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"original_title"	TEXT,
	"publication_date"	BIGINT,
	"notes"	TEXT,
	"type_id"	INTEGER,
	"pre_accepted"	INTEGER DEFAULT (1),
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("type_id") REFERENCES "types"("id")
);
CREATE TABLE IF NOT EXISTS "contents_current_languages" (
	"content_id"	INTEGER NOT NULL,
	"curr_lang_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","curr_lang_id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id"),
	FOREIGN KEY("curr_lang_id") REFERENCES "current_languages"("id")
);
CREATE TABLE IF NOT EXISTS "contents_original_languages" (
	"content_id"	INTEGER NOT NULL,
	"orig_lang_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","orig_lang_id"),
	FOREIGN KEY("orig_lang_id") REFERENCES "original_languages"("id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id")
);
CREATE TABLE IF NOT EXISTS "contents_people_roles" (
	"content_id"	INTEGER NOT NULL,
	"person_id"	INTEGER NOT NULL,
	"role_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","person_id","role_id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id"),
	FOREIGN KEY("role_id") REFERENCES "roles"("id"),
	FOREIGN KEY("person_id") REFERENCES "people"("id")
);
CREATE TABLE IF NOT EXISTS "contents_source_languages" (
	"content_id"	INTEGER NOT NULL,
	"source_lang_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","source_lang_id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id"),
	FOREIGN KEY("source_lang_id") REFERENCES "source_languages"("id")
);
CREATE TABLE IF NOT EXISTS "contents_tags" (
	"content_id"	INTEGER NOT NULL,
	"tag_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","tag_id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id"),
	FOREIGN KEY("tag_id") REFERENCES "tags"("id")
);
CREATE TABLE IF NOT EXISTS "formats" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "laverdure" (
	"key"	TEXT NOT NULL,
	"value"	TEXT,
	PRIMARY KEY("key")
);
CREATE TABLE IF NOT EXISTS "people" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	"nationality"	TEXT,
	"birth_date"	INTEGER,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "publishers" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "roles" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "series" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "tags" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "types" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "languages_names" (
	"id"	INTEGER,
	"iso_code"	TEXT NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "original_languages" (
	"content_id"	INTEGER NOT NULL,
	"language_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","language_id"),
	FOREIGN KEY("language_id") REFERENCES "languages_names"("id")
);
CREATE TABLE IF NOT EXISTS "current_languages" (
	"content_id"	INTEGER NOT NULL,
	"language_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","language_id"),
	FOREIGN KEY("language_id") REFERENCES "languages_names"("id")
);
CREATE TABLE IF NOT EXISTS "source_languages" (
	"content_id"	INTEGER NOT NULL,
	"language_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","language_id"),
	FOREIGN KEY("language_id") REFERENCES "languages_names"("id")
);
CREATE INDEX IF NOT EXISTS "idx_books_contents_junction" ON "books_contents" (
	"book_id",
	"content_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_core_search" ON "books" (
	"name",
	"series_id",
	"publication_date"
);
CREATE INDEX IF NOT EXISTS "idx_books_metadata" ON "books" (
	"publisher_id",
	"format_id",
	"series_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_people_lookup" ON "books_people_roles" (
	"book_id",
	"person_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_series_index" ON "books" (
	"series_id",
	"series_index"
);
CREATE INDEX IF NOT EXISTS "idx_books_tags_lookup" ON "books_tags" (
	"book_id",
	"tag_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_temporal" ON "books" (
	"publication_date",
	"acquisition_date",
	"last_modified_date"
);
CREATE INDEX IF NOT EXISTS "idx_contents_core_search" ON "contents" (
	"name",
	"type_id",
	"publication_date"
);
CREATE INDEX IF NOT EXISTS "idx_contents_languages" ON "contents_current_languages" (
	"content_id",
	"curr_lang_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_metadata" ON "contents" (
	"type_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_original_languages" ON "contents_original_languages" (
	"content_id",
	"orig_lang_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_people_lookup" ON "contents_people_roles" (
	"content_id",
	"person_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_source_languages" ON "contents_source_languages" (
	"content_id",
	"source_lang_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_tags_lookup" ON "contents_tags" (
	"content_id",
	"tag_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_temporal" ON "contents" (
	"publication_date"
);
CREATE INDEX IF NOT EXISTS "idx_contents_type_date" ON "contents" (
	"type_id",
	"publication_date"
);
CREATE INDEX IF NOT EXISTS "idx_people_search" ON "people" (
	"name",
	"id"
);
CREATE INDEX IF NOT EXISTS "idx_publishers_search" ON "publishers" (
	"name",
	"id"
);
CREATE INDEX IF NOT EXISTS "idx_roles_search" ON "roles" (
	"name"
);
CREATE INDEX IF NOT EXISTS "idx_series_search" ON "series" (
	"name"
);
CREATE INDEX IF NOT EXISTS "idx_tags_search" ON "tags" (
	"name"
);
CREATE INDEX IF NOT EXISTS "idx_v_contents_details" ON "contents" (
	"id",
	"type_id",
	"publication_date"
);
