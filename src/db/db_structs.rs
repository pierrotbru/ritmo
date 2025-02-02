pub struct People {
	name: String,
	nationality: Option<String>,
	birth_year: Option<String>
}

#[derive(Debug, sqlx::FromRow)]
pub struct BookDetails {
    pub book_id: i64,
    pub book_title: String,
    pub book_original_title: Option<String>,
    pub publication_date: Option<i64>,
    pub acquisition_date: Option<i64>,
    pub last_modified_date: Option<i64>,
    pub book_notes: Option<String>,
    pub publisher_name: Option<String>,
    pub format_name: Option<String>,
    pub series_name: Option<String>,
    pub series_id: Option<i64>,
    pub series_index: Option<u32>,
    pub book_tags: Option<String>,
    pub book_people: Option<String>,
}

impl BookDetails {
    pub fn get_tags(&self) -> Vec<String> {
        self.book_tags
            .as_ref()
            .map(|tags| tags.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }

    pub fn get_people(&self) -> Vec<String> {
        self.book_people
            .as_ref()
            .map(|people| people.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct ContentDetails {
    pub content_id: i64,
    pub content_title: String,
    pub content_original_title: Option<String>,
    pub issue_date: Option<i64>,
    pub content_notes: Option<String>,
    pub type_name: Option<String>,
    pub content_tags: Option<String>,
    pub content_people: Option<String>,
    pub content_current_languages: Option<String>,
    pub content_original_languages: Option<String>,
    pub content_source_languages: Option<String>,
}

impl ContentDetails {
    pub fn get_tags(&self) -> Vec<String> {
        self.content_tags
            .as_ref()
            .map(|tags| tags.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }

    pub fn get_people(&self) -> Vec<String> {
        self.content_people
            .as_ref()
            .map(|people| people.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }

    pub fn get_current_languages(&self) -> Vec<String> {
        self.content_current_languages
            .as_ref()
            .map(|langs| langs.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }

    pub fn get_original_languages(&self) -> Vec<String> {
        self.content_original_languages
            .as_ref()
            .map(|langs| langs.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }

    pub fn get_source_languages(&self) -> Vec<String> {
        self.content_source_languages
            .as_ref()
            .map(|langs| langs.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct BookPeopleDetails {
    pub book_id: i64,
    pub book_title: String,
    pub person_id: i64,
    pub person_name: String,
    pub role_name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ContentPeopleDetails {
    pub content_id: i64,
    pub content_title: String,
    pub person_id: i64,
    pub person_name: String,
    pub role_name: String,
}
