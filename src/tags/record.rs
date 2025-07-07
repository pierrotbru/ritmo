use serde::{Deserialize, Serialize};

use crate::people::record::MLEntity;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TagRecord {
    pub id: i64,
    pub label: String,
    pub normalized_label: String,
    // altri campi: category, description, ecc.
}

impl TagRecord {
    pub fn new(id: i64, label: &str) -> Self {
        let normalized_label = Self::normalize(label);
        Self {
            id,
            label: label.to_string(),
            normalized_label,
        }
    }

    pub fn normalize(label: &str) -> String {
        label.to_lowercase().replace(|c: char| !c.is_alphanumeric(), "")
    }
}

impl MLEntity for TagRecord {
    fn id(&self) -> i64 {
        self.id
    }
    fn key(&self) -> &str {
        &self.label
    }
    fn normalized_key(&self) -> String {
        self.normalized_label.clone()
    }
}