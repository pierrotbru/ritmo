use serde::{Deserialize, Serialize};

use crate::people::record::MLEntity;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SeriesRecord {
    pub id: i64,
    pub title: String,
    pub normalized_title: String,
    // altri campi: year, description, ecc.
}

impl SeriesRecord {
    pub fn new(id: i64, title: &str) -> Self {
        let normalized_title = Self::normalize(title);
        Self {
            id,
            title: title.to_string(),
            normalized_title,
        }
    }

    pub fn normalize(title: &str) -> String {
        title.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "")
    }
}

//impl MLEntity for SeriesRecord {
//    fn id(&self) -> i64 {
//        self.id
//    }
//    fn key(&self) -> &str {
//        &self.title
//    }
//    fn normalized_key(&self) -> String {
//        self.normalized_title.clone()
//    }
//}