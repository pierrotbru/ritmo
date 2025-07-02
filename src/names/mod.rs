// src/name_matching/mod.rs

pub mod models;
pub mod utils;
pub mod manager;
pub mod matching;
pub mod ml;
pub mod persistence;
pub mod process;
pub mod names_ml;
pub mod names_feedback;

// Re-export common types
pub use models::*;
pub use manager::NameManager;