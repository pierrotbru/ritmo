use std::collections::HashSet;
use crate::ml::traits::MLProcessable;
use crate::ml::feedback::Feedback;
use crate::ml::generic::{apply_negative_feedback, apply_positive_feedback};

// Struct di test che implementa MLProcessable
#[derive(Clone, Debug, PartialEq)]
struct TestRecord {
    id: i64,
    canonical: String,
    variants: Vec<String>,
}

impl MLProcessable for TestRecord {
    fn id(&self) -> i64 { self.id }
    fn canonical_key(&self) -> &str { &self.canonical }
    fn variants(&self) -> Vec<String> { self.variants.clone() }
    fn set_variants(&mut self, variants: Vec<String>) { self.variants = variants; }
}

#[test]
fn test_apply_negative_feedback() {
    // Setup: due record con varianti incrociate
    let mut records = vec![
        TestRecord { id: 1, canonical: "pippo".into(), variants: vec!["pippo".into(), "pluto".into()] },
        TestRecord { id: 2, canonical: "pluto".into(), variants: vec!["pluto".into(), "pippo".into()] },
    ];

    let mut feedback = Feedback::new();
    feedback.add_negative("pippo", "pluto");

    apply_negative_feedback(&mut records, &feedback);

    assert_eq!(records[0].variants, vec!["pippo".to_string()]);
    assert_eq!(records[1].variants, vec!["pluto".to_string()]);
}

#[test]
fn test_apply_positive_feedback() {
    // Setup: due record con varianti diverse
    let mut records = vec![
        TestRecord { id: 1, canonical: "pippo".into(), variants: vec!["pippo".into()] },
        TestRecord { id: 2, canonical: "pluto".into(), variants: vec!["pluto".into()] },
    ];

    let mut feedback = Feedback::new();
    feedback.add_positive("pippo", "pluto");

    apply_positive_feedback(&mut records, &feedback);

    // Entrambi devono avere entrambe le varianti
    assert_eq!(records[0].variants, vec!["pippo".to_string(), "pluto".to_string()]);
    assert_eq!(records[1].variants, vec!["pippo".to_string(), "pluto".to_string()]);
}

#[test]
fn test_feedback_combined() {
    // Setup: tre record, alcuni merge positivi, altri negativi
    let mut records = vec![
        TestRecord { id: 1, canonical: "pippo".into(), variants: vec!["pippo".into()] },
        TestRecord { id: 2, canonical: "pluto".into(), variants: vec!["pluto".into()] },
        TestRecord { id: 3, canonical: "paperino".into(), variants: vec!["paperino".into(), "pluto".into()] },
    ];

    let mut feedback = Feedback::new();
    feedback.add_positive("pippo", "pluto");
    feedback.add_negative("pluto", "paperino");

    // Applico prima i positivi, poi i negativi
    apply_positive_feedback(&mut records, &feedback);
    apply_negative_feedback(&mut records, &feedback);

    // pippo e pluto condividono varianti, paperino NO
    assert_eq!(records[0].variants, vec!["pippo".to_string(), "pluto".to_string()]);
    assert_eq!(records[1].variants, vec!["pippo".to_string(), "pluto".to_string()]);
    assert_eq!(records[2].variants, vec!["paperino".to_string()]);
}