DROP INDEX IF  EXISTS idx_contents_core_search;
DROP INDEX IF  EXISTS idx_books_core_search;

-- People and Role Relationship Indices
DROP INDEX IF  EXISTS idx_books_people_lookup;
DROP INDEX IF  EXISTS idx_contents_people_lookup;

-- Language Indices for Multilingual Support
DROP INDEX IF  EXISTS idx_contents_languages;
DROP INDEX IF  EXISTS idx_contents_original_languages;
DROP INDEX IF  EXISTS idx_contents_source_languages;

-- Tagging and Categorization Indices
DROP INDEX IF  EXISTS idx_contents_tags_lookup;
DROP INDEX IF  EXISTS idx_books_tags_lookup;

-- Metadata and Filtering Indices
DROP INDEX IF  EXISTS idx_books_metadata;
DROP INDEX IF  EXISTS idx_contents_metadata;

-- Temporal Indices for Date-based Queries
DROP INDEX IF  EXISTS idx_books_temporal;
DROP INDEX IF  EXISTS idx_contents_temporal;

-- Junction Table Performance Indices
DROP INDEX IF  EXISTS idx_books_contents_junction;

-- Search and Filtering Optimization
DROP INDEX IF  EXISTS idx_people_search;
DROP INDEX IF  EXISTS idx_publishers_search;
DROP INDEX IF  EXISTS idx_series_search;
DROP INDEX IF  EXISTS idx_tags_search;
DROP INDEX IF  EXISTS idx_roles_search;

-- Composite Indices for Complex Queries
DROP INDEX IF  EXISTS idx_books_series_index;
DROP INDEX IF  EXISTS idx_contents_type_date;

-- View Support Index
DROP INDEX IF  EXISTS idx_v_contents_details;