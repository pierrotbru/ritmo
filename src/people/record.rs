use crate::ml::traits::MLEntity;
use human_name::HumanName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PersonRecord {
    pub id: i64,
    pub full_name: String,
    pub normalized_name: String,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    // altri campi se necessario
}

impl PersonRecord {
    // Crea un nuovo record usando human-name per parsing e normalizzazione
    pub fn new(id: i64, full_name: &str) -> Self {
        let parsed = HumanName::parse(full_name);
        let normalized_name = parsed.normalized(); // Usa la normalizzazione "umana"
        let given_name = parsed.given_name().map(|s| s.to_string());
        let surname = parsed.surname().map(|s| s.to_string());
        Self {
            id,
            full_name: full_name.to_string(),
            normalized_name,
            given_name,
            surname,
        }
    }
}

impl MLEntity for PersonRecord {
    fn id(&self) -> i64 {
        self.id
    }
    fn key(&self) -> &str {
        &self.full_name
    }
    fn normalized_key(&self) -> String {
        self.normalized_name.clone()
    }
}
