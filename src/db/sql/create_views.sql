CREATE VIEW v_books_people_details AS
SELECT
    b.id AS book_id,
    b.title AS book_title,
    p.id AS person_id,
    p.name AS person_name,
    r.name AS role_name
FROM books AS b
JOIN books_people AS bp ON b.id = bp.book_id
JOIN people AS p ON bp.person_id = p.id
JOIN roles AS r ON bp.role_id = r.id;

CREATE VIEW v_contents_people_details AS
SELECT
    c.id AS content_id,
    c.title AS content_title,
    p.id AS person_id,
    p.name AS person_name,
    r.name AS role_name
FROM contents AS c
JOIN contents_people AS cp ON c.id = cp.content_id
JOIN people AS p ON cp.person_id = p.id
JOIN roles AS r ON cp.role_id = r.id;

CREATE VIEW v_books_details AS
SELECT
    b.id AS book_id,
    b.title AS book_title,
    b.original_title AS book_original_title,
    b.publication_date,
    b.acquisition_date,
    b.last_modified_date,
    b.notes AS book_notes,
    p.name AS publisher_name,
    f.format_name,
    s.series AS series_name,
    s.series_id,
    b.series_index,
    GROUP_CONCAT(DISTINCT t.tag_name) AS book_tags,
    GROUP_CONCAT(DISTINCT pe.name || ' (' || r.name || ')') AS book_people
FROM books AS b
LEFT JOIN publishers AS p ON b.publisher_id = p.id
LEFT JOIN formats AS f ON b.format_id = f.id
LEFT JOIN series AS s ON b.series_id = s.series_id
LEFT JOIN books_tags AS bt ON b.id = bt.book_id
LEFT JOIN tags AS t ON bt.tag_id = t.id
LEFT JOIN books_people AS bp ON b.id = bp.book_id
LEFT JOIN people AS pe ON bp.person_id = pe.id
LEFT JOIN roles AS r ON bp.role_id = r.id
GROUP BY b.id;

CREATE VIEW v_contents_details AS
SELECT
    c.id AS content_id,
    c.title AS content_title,
    c.original_title AS content_original_title,
    c.publication_date AS issue_date,
    c.notes AS content_notes,
    ct.type_name,
    GROUP_CONCAT(DISTINCT t.tag_name) AS content_tags,
    GROUP_CONCAT(DISTINCT pe.name || ' (' || r.name || ')') AS content_people,
    GROUP_CONCAT(DISTINCT cl.lang_code) AS content_current_languages,
    GROUP_CONCAT(DISTINCT ol.lang_code) AS content_original_languages,
    GROUP_CONCAT(DISTINCT sl.lang_code) AS content_source_languages
FROM contents AS c
LEFT JOIN contents_types AS ct ON c.type_id = ct.id
LEFT JOIN contents_tags AS ctg ON c.id = ctg.content_id
LEFT JOIN tags AS t ON ctg.tag_id = t.id
LEFT JOIN contents_people AS cp ON c.id = cp.content_id
LEFT JOIN people AS pe ON cp.person_id = pe.id
LEFT JOIN roles AS r ON cp.role_id = r.id
LEFT JOIN contents_current_languages AS ccl ON c.id = ccl.content_id
LEFT JOIN current_languages AS cl ON ccl.curr_lang_id = cl.id
LEFT JOIN contents_original_languages AS col ON c.id = col.content_id
LEFT JOIN original_languages AS ol ON col.orig_lang_id = ol.id
LEFT JOIN contents_source_languages AS csl ON c.id = csl.content_id
LEFT JOIN source_languages AS sl ON csl.source_lang_id = sl.id
GROUP BY c.id;

CREATE VIEW v_books_with_contents AS
SELECT
    bd.*,
    GROUP_CONCAT(DISTINCT cd.content_title) as contents_titles,
    GROUP_CONCAT(DISTINCT cd.content_id) as contents_ids
FROM v_books_details bd
LEFT JOIN books_contents bc ON bd.book_id = bc.book_id
LEFT JOIN v_contents_details cd ON bc.content_id = cd.content_id
GROUP BY bd.book_id;

CREATE VIEW v_full_books AS
SELECT
    b.id AS book_id,
    b.title AS book_title,
    b.original_title AS book_original_title,
    b.publication_date,
    b.acquisition_date,
    b.last_modified_date,
    b.notes AS book_notes,
    p.name AS publisher_name,
    f.format_name,
    s.series AS series_name,
    s.series_id,
    b.series_index,
    GROUP_CONCAT(DISTINCT t.tag_name) AS book_tags,
    GROUP_CONCAT(DISTINCT pe.name || ' (' || r.name || ')') AS book_people,
    GROUP_CONCAT(DISTINCT c.title) AS content_titles,  -- Titoli dei contenuti
    GROUP_CONCAT(DISTINCT c.original_title) AS content_original_titles, -- Titoli originali dei contenuti
    GROUP_CONCAT(DISTINCT c.publication_date) AS content_publication_dates, -- Date di pubblicazione dei contenuti
    GROUP_CONCAT(DISTINCT c.notes) AS content_notes, -- Note dei contenuti
    GROUP_CONCAT(DISTINCT ct.type_name) AS content_types, -- Tipi di contenuto
    GROUP_CONCAT(DISTINCT cl.lang_code) AS content_current_languages, -- Lingue correnti dei contenuti
    GROUP_CONCAT(DISTINCT ol.lang_code) AS content_original_languages, -- Lingue originali dei contenuti
    GROUP_CONCAT(DISTINCT sl.lang_code) AS content_source_languages -- Lingue sorgente dei contenuti
FROM books AS b
LEFT JOIN publishers AS p ON b.publisher_id = p.id
LEFT JOIN formats AS f ON b.format_id = f.id
LEFT JOIN series AS s ON b.series_id = s.series_id
LEFT JOIN books_tags AS bt ON b.id = bt.book_id
LEFT JOIN tags AS t ON bt.tag_id = t.id
LEFT JOIN books_people AS bp ON b.id = bp.book_id
LEFT JOIN people AS pe ON bp.person_id = pe.id
LEFT JOIN roles AS r ON bp.role_id = r.id
LEFT JOIN books_contents AS bc ON b.id = bc.book_id
LEFT JOIN contents AS c ON bc.content_id = c.id
LEFT JOIN contents_types AS ct ON c.type_id = ct.id
LEFT JOIN contents_current_languages AS ccl ON c.id = ccl.content_id
LEFT JOIN current_languages AS cl ON ccl.curr_lang_id = cl.id
LEFT JOIN contents_original_languages AS col ON c.id = col.content_id
LEFT JOIN original_languages AS ol ON col.orig_lang_id = ol.id
LEFT JOIN contents_source_languages AS csl ON c.id = csl.content_id
LEFT JOIN source_languages AS sl ON csl.source_lang_id = sl.id
GROUP BY b.id;

CREATE VIEW v_full_contents AS
SELECT
    c.id AS content_id,
    c.title AS content_title,
    c.original_title AS content_original_title,
    c.publication_date AS issue_date,
    c.notes AS content_notes,
    ct.type_name,
    GROUP_CONCAT(DISTINCT t.tag_name) AS content_tags,
    GROUP_CONCAT(DISTINCT pe.name || ' (' || r.name || ')') AS content_people,
    GROUP_CONCAT(DISTINCT cl.lang_code) AS content_current_languages,
    GROUP_CONCAT(DISTINCT ol.lang_code) AS content_original_languages,
    GROUP_CONCAT(DISTINCT sl.lang_code) AS content_source_languages,
    b.id AS book_id,  -- ID del libro
    b.title AS book_title,  -- Titolo del libro
    b.original_title AS book_original_title  -- Titolo originale del libro
FROM contents AS c
LEFT JOIN contents_types AS ct ON c.type_id = ct.id
LEFT JOIN contents_tags AS ctg ON c.id = ctg.content_id
LEFT JOIN tags AS t ON ctg.tag_id = t.id
LEFT JOIN contents_people AS cp ON c.id = cp.content_id
LEFT JOIN people AS pe ON cp.person_id = pe.id
LEFT JOIN roles AS r ON cp.role_id = r.id
LEFT JOIN contents_current_languages AS ccl ON c.id = ccl.content_id
LEFT JOIN current_languages AS cl ON ccl.curr_lang_id = cl.id
LEFT JOIN contents_original_languages AS col ON c.id = col.content_id
LEFT JOIN original_languages AS ol ON col.orig_lang_id = ol.id
LEFT JOIN contents_source_languages AS csl ON c.id = csl.content_id
LEFT JOIN source_languages AS sl ON csl.source_lang_id = sl.id
LEFT JOIN books_contents AS bc ON c.id = bc.content_id  -- JOIN con la tabella books_contents
LEFT JOIN books AS b ON bc.book_id = b.id  -- JOIN con la tabella books
GROUP BY c.id;
