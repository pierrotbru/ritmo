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
CREATE TABLE IF NOT EXISTS "contents_source_languages" (
	"content_id"	INTEGER NOT NULL,
	"source_lang_id"	INTEGER NOT NULL,
	PRIMARY KEY("content_id","source_lang_id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id"),
	FOREIGN KEY("source_lang_id") REFERENCES "source_languages"("id")
);

