use std::error::Error;
use serde::{Deserialize, Serialize};
use crate::errors::RitmoErr;

#[derive(Debug)]
pub enum NameManagerErrorInternal {
    NameParsingError(String),
}

impl std::fmt::Display for NameManagerErrorInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NameManagerErrorInternal::NameParsingError(msg) => write!(f, "Errore di parsificazione nome interno: {}", msg),
        }
    }
}

impl Error for NameManagerErrorInternal {}

impl From<NameManagerErrorInternal> for RitmoErr {
    fn from(err: NameManagerErrorInternal) -> Self {
        match err {
            NameManagerErrorInternal::NameParsingError(msg) => RitmoErr::NameParsingError(msg),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonRecord {
    pub id: i64,
    pub original_input: String,
    pub parsed_name: ParsedName,
    pub normalized_key: String,
    pub confidence: f64,
    pub verified: bool,
    pub aliases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ParsedName {
    pub given_name: String,
    pub surname: String,
    pub middle_names: Vec<String>,
    pub title: Option<String>,
    pub suffix: Option<String>,
    pub display_name: String,
}

#[derive(Debug)]
pub enum MatchResult {
    ExactMatch(i64),
    HighConfidenceMatch(Vec<NameMatch>),
    PossibleMatches(Vec<NameMatch>),
    NoMatch,
}

#[derive(Debug, Clone)]
pub struct NameMatch {
    pub person_id: i64,
    pub matched_name: String,
    pub similarity_score: f64,
    pub match_type: MatchType,
    pub confidence: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchType {
    Exact,
    NameOrder,
    Typo,
    Alias,
    TypoMinor,
    TypoMajor,
    Learned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NameVariantPattern {
    pub base_form: String,
    pub variant_form: String,
    pub pattern_type: VariantPatternType,
    pub confidence: f64,
    pub frequency: usize,
    pub edit_distance: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Copy)]
pub enum VariantPatternType {
    Suffix,        
    Prefix,        
    Transliteration, 
    Abbreviation,  
    Compound,      
    Typo,
    Other
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NameCluster {
    pub cluster_id: usize,
    pub members: Vec<String>,
    pub centroid: String,
    pub confidence: f64,
}
