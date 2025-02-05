-- Comprehensive Performance Indices for Ritmo Database

-- Core Content and Book Lookup Indices
CREATE INDEX IF NOT EXISTS idx_contents_core_search ON contents (title, type_id, publication_date);
CREATE INDEX IF NOT EXISTS idx_books_core_search ON books (title, series_id, publication_date);

-- People and Role Relationship Indices
CREATE INDEX IF NOT EXISTS idx_books_people_lookup ON books_people (book_id, person_id, role_id);
CREATE INDEX IF NOT EXISTS idx_contents_people_lookup ON contents_people (content_id, person_id, role_id);

-- Language Indices for Multilingual Support
CREATE INDEX IF NOT EXISTS idx_contents_languages ON contents_current_languages (content_id, curr_lang_id);
CREATE INDEX IF NOT EXISTS idx_contents_original_languages ON contents_original_languages (content_id, orig_lang_id);
CREATE INDEX IF NOT EXISTS idx_contents_source_languages ON contents_source_languages (content_id, source_lang_id);

-- Tagging and Categorization Indices
CREATE INDEX IF NOT EXISTS idx_contents_tags_lookup ON contents_tags (content_id, tag_id);
CREATE INDEX IF NOT EXISTS idx_books_tags_lookup ON books_tags (book_id, tag_id);

-- Metadata and Filtering Indices
CREATE INDEX IF NOT EXISTS idx_books_metadata ON books (publisher_id, format_id, series_id);
CREATE INDEX IF NOT EXISTS idx_contents_metadata ON contents (type_id);

-- Temporal Indices for Date-based Queries
CREATE INDEX IF NOT EXISTS idx_books_temporal ON books (publication_date, acquisition_date, last_modified_date);
CREATE INDEX IF NOT EXISTS idx_contents_temporal ON contents (publication_date);

-- Junction Table Performance Indices
CREATE INDEX IF NOT EXISTS idx_books_contents_junction ON books_contents (book_id, content_id);

-- Search and Filtering Optimization
CREATE INDEX IF NOT EXISTS idx_people_search ON people (name);
CREATE INDEX IF NOT EXISTS idx_publishers_search ON publishers (name);
CREATE INDEX IF NOT EXISTS idx_series_search ON series (series);
CREATE INDEX IF NOT EXISTS idx_tags_search ON tags (tag_name);
CREATE INDEX IF NOT EXISTS idx_roles_search ON roles (name);

-- Composite Indices for Complex Queries
CREATE INDEX IF NOT EXISTS idx_books_series_index ON books (series_id, series_index);
CREATE INDEX IF NOT EXISTS idx_contents_type_date ON contents (type_id, publication_date);

-- View Support Index
CREATE INDEX IF NOT EXISTS idx_v_contents_details ON contents(id, type_id, publication_date);