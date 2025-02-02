CREATE VIEW BookPeopleDetails AS
SELECT
    b.id AS book_id,
    b.title AS book_title,
    p.id AS person_id,
    p.name AS person_name,
    r.role_name AS role_name
FROM Books AS b
JOIN BookPeople AS bp ON b.id = bp.book_id
JOIN People AS p ON bp.person_id = p.id
JOIN Role AS r ON bp.role_id = r.id;

    
CREATE VIEW ContentPeopleDetails AS
SELECT
    c.id AS content_id,
    c.title AS content_title,
    p.id AS person_id,
    p.name AS person_name,
    r.role_name AS role_name
FROM Content AS c
JOIN ContentPeople AS cp ON c.id = cp.content_id
JOIN People AS p ON cp.person_id = p.id
JOIN Role AS r ON cp.role_id = r.id;

CREATE VIEW BookDetails AS
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
    GROUP_CONCAT(DISTINCT pe.name || ' (' || r.role_name || ')') AS book_people
FROM Books AS b
LEFT JOIN Publishers AS p ON b.publisher_id = p.id
LEFT JOIN Formats AS f ON b.format_id = f.id
LEFT JOIN Series AS s ON b.series_id = s.series_id
LEFT JOIN BooksTags AS bt ON b.id = bt.book_id
LEFT JOIN Tags AS t ON bt.tag_id = t.id
LEFT JOIN BookPeople AS bp ON b.id = bp.book_id
LEFT JOIN People AS pe ON bp.person_id = pe.id
LEFT JOIN Role AS r ON bp.role_id = r.id
GROUP BY b.id;

CREATE VIEW ContentDetails AS
SELECT
    c.id AS content_id,
    c.title AS content_title,
    c.original_title AS content_original_title,
    c.issue_date,
    c.notes AS content_notes,
    ct.type_name,
    GROUP_CONCAT(DISTINCT t.tag_name) AS content_tags,
    GROUP_CONCAT(DISTINCT pe.name || ' (' || r.role_name || ')') AS content_people,
    GROUP_CONCAT(DISTINCT cl.lang_code) AS content_current_languages,
    GROUP_CONCAT(DISTINCT ol.lang_code) AS content_original_languages,
    GROUP_CONCAT(DISTINCT sl.lang_code) AS content_source_languages
FROM Content AS c
LEFT JOIN ContentTypes AS ct ON c.type_id = ct.id
LEFT JOIN ContentTags AS ctg ON c.id = ctg.content_id
LEFT JOIN Tags AS t ON ctg.tag_id = t.id
LEFT JOIN ContentPeople AS cp ON c.id = cp.content_id
LEFT JOIN People AS pe ON cp.person_id = pe.id
LEFT JOIN Role AS r ON cp.role_id = r.id
LEFT JOIN ContentCurrLang AS ccl ON c.id = ccl.content_id
LEFT JOIN CurrentLanguages AS cl ON ccl.curr_lang_id = cl.id
LEFT JOIN ContentOrigLang AS col ON c.id = col.content_id
LEFT JOIN OriginalLanguages AS ol ON col.orig_lang_id = ol.id
LEFT JOIN ContentSourcLang AS csl ON c.id = csl.content_id
LEFT JOIN SourceLanguages AS sl ON csl.source_lang_id = sl.id
GROUP BY c.id;

CREATE VIEW BookWithContents AS
SELECT
    bd.*,
    GROUP_CONCAT(DISTINCT cd.content_title) as contents_titles,
    GROUP_CONCAT(DISTINCT cd.content_id) as contents_ids
FROM BookDetails bd
LEFT JOIN BookContents bc ON bd.book_id = bc.book_id
LEFT JOIN ContentDetails cd ON bc.content_id = cd.content_id
GROUP BY bd.book_id;