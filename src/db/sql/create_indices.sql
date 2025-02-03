-- Comprehensive Indices for Performance Optimization

-- books Table Indices
CREATE INDEX IF NOT EXISTS idx_books_publication_date_range ON books (publication_date);
CREATE INDEX IF NOT EXISTS idx_books_series_lookup ON books (series_id, series_index);

-- Composite Indices for Frequent Joins and Filters
CREATE INDEX IF NOT EXISTS idx_books_publisher_format ON books (publisher_id, format_id);
CREATE INDEX IF NOT EXISTS idx_books_acquisition_modified ON books (acquisition_date, last_modified_date);

-- -- Junction Tables Indices (for Performance in Joins)
-- CREATE INDEX IF NOT EXISTS idx_bookcontents_content_lookup ON books_contents (content_id, book_id);
-- CREATE INDEX IF NOT EXISTS idx_bookpeople_person_role ON books_people (person_id, role_id);
-- CREATE INDEX IF NOT EXISTS idx_contentpeople_person_role ON contents_people (person_id, role_id);
-- 
-- -- Language Indices
-- CREATE INDEX IF NOT EXISTS idx_content_languages ON contents_current_languages (curr_lang_id);
-- CREATE INDEX IF NOT EXISTS idx_content_orig_languages ON contents_original_languages (orig_lang_id);
-- CREATE INDEX IF NOT EXISTS idx_content_source_languages ON contents_source_languages (source_lang_id);
-- 
-- -- tags and Categorization Indices
-- CREATE INDEX IF NOT EXISTS idx_contenttags_tag_lookup ON contents_tags (tag_id);
-- CREATE INDEX IF NOT EXISTS idx_bookstags_tag_lookup ON books_tags (tag_id);
-- 
-- -- Metadata Indices
-- CREATE INDEX IF NOT EXISTS idx_publishers_name_search ON publishers (name);
-- CREATE INDEX IF NOT EXISTS idx_series_name_search ON series (series);
-- CREATE INDEX IF NOT EXISTS idx_contenttypes_name ON contents_types (type_name);
-- CREATE INDEX IF NOT EXISTS idx_formats_name ON formats (format_name);
-- CREATE INDEX IF NOT EXISTS idx_tags_name ON tags (tag_name);
-- CREATE INDEX IF NOT EXISTS idx_role_name ON roles (role_name);
-- 