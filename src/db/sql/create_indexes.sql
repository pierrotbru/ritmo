-- Comprehensive Indices for Performance Optimization

-- Books Table Indices
CREATE INDEX IF NOT EXISTS idx_books_publication_date_range ON Books (publication_date);
CREATE INDEX IF NOT EXISTS idx_books_series_lookup ON Books (series_id, series_index);

-- Composite Indices for Frequent Joins and Filters
CREATE INDEX IF NOT EXISTS idx_books_publisher_format ON Books (publisher_id, format_id);
CREATE INDEX IF NOT EXISTS idx_books_acquisition_modified ON Books (acquisition_date, last_modified_date);

-- Content Table Indices
CREATE INDEX IF NOT EXISTS idx_content_type_date ON Content (type_id, issue_date);

-- People and Roles Indices
CREATE INDEX IF NOT EXISTS idx_people_nationality ON People (nationality);

-- Junction Tables Indices (for Performance in Joins)
CREATE INDEX IF NOT EXISTS idx_bookcontents_content_lookup ON BookContents (content_id, book_id);
CREATE INDEX IF NOT EXISTS idx_bookpeople_person_role ON BookPeople (person_id, role_id);
CREATE INDEX IF NOT EXISTS idx_contentpeople_person_role ON ContentPeople (person_id, role_id);

-- Language Indices
CREATE INDEX IF NOT EXISTS idx_content_languages ON ContentCurrLang (curr_lang_id);
CREATE INDEX IF NOT EXISTS idx_content_orig_languages ON ContentOrigLang (orig_lang_id);
CREATE INDEX IF NOT EXISTS idx_content_source_languages ON ContentSourcLang (source_lang_id);

-- Tags and Categorization Indices
CREATE INDEX IF NOT EXISTS idx_contenttags_tag_lookup ON ContentTags (tag_id);
CREATE INDEX IF NOT EXISTS idx_bookstags_tag_lookup ON BooksTags (tag_id);

-- Metadata Indices
CREATE INDEX IF NOT EXISTS idx_publishers_name_search ON Publishers (name);
CREATE INDEX IF NOT EXISTS idx_series_name_search ON Series (series);
CREATE INDEX IF NOT EXISTS idx_contenttypes_name ON ContentTypes (type_name);
CREATE INDEX IF NOT EXISTS idx_formats_name ON Formats (format_name);
CREATE INDEX IF NOT EXISTS idx_tags_name ON Tags (tag_name);
CREATE INDEX IF NOT EXISTS idx_role_name ON Role (role_name);
