use std::collections::HashMap;
use crate::{RitmoErr};
use sqlx::SqlitePool;
use strsim::jaro_winkler;

#[derive(Clone, Debug)]
pub struct PublisherCluster {
    pub centroid: String,
    pub members: Vec<String>,
    pub confidence: f64,
}

#[derive(Clone, Debug)]
pub struct PublisherPattern {
    pub base_form: String,
    pub variant_form: String,
    pub pattern_type: String,
    pub confidence: f64,
}

pub struct MLPublisherLearner {
    pub publisher_clusters: Vec<PublisherCluster>,
    pub learned_patterns: Vec<PublisherPattern>,
    pub pattern_frequency: HashMap<String, u32>,
    // eventuali altri campi (configurazioni, soglie, ecc.)
}

impl MLPublisherLearner {
    pub fn new() -> Self {
        Self {
            publisher_clusters: Vec::new(),
            learned_patterns: Vec::new(),
            pattern_frequency: HashMap::new(),
        }
    }

    pub fn create_publisher_clusters(&mut self, publishers: &[String]) {
        let mut clusters = Vec::new();
        let mut used = vec![false; publishers.len()];
        let threshold = 0.85; // Soglia di similarità

        for (i, pub1) in publishers.iter().enumerate() {
            if used[i] { continue; }
            let mut group = vec![pub1.clone()];
            used[i] = true;
            for (j, pub2) in publishers.iter().enumerate().skip(i + 1) {
                if !used[j] && jaro_winkler(pub1, pub2) > threshold {
                    group.push(pub2.clone());
                    used[j] = true;
                }
            }
            if group.len() > 1 {
                let centroid = find_centroid(&group);
                let confidence = calc_group_confidence(&group);
                clusters.push(PublisherCluster { centroid, members: group, confidence });
            }
        }
        self.publisher_clusters = clusters;
    }

    pub fn identify_variant_patterns(&mut self) {
        // Placeholder: implementa logica di estrazione pattern tra varianti come per i nomi
        // Popola self.learned_patterns e self.pattern_frequency
    }

    pub async fn save_to_db(&self, pool: &SqlitePool) -> Result<(), RitmoErr> {
        // Serializza e salva i dati (clusters, patterns, frequenze) nel db, come già fai per i nomi
        Ok(())
    }
}

// Funzioni di utilità (possono essere spostate in moduli comuni)
fn find_centroid(group: &[String]) -> String {
    // Sceglie il nome più rappresentativo del gruppo (es. quello con la lunghezza media)
    let mut min_dist_sum = std::f64::MAX;
    let mut centroid = group[0].clone();
    for candidate in group {
        let sum = group.iter().map(|other| 1.0 - jaro_winkler(candidate, other)).sum();
        if sum < min_dist_sum {
            min_dist_sum = sum;
            centroid = candidate.clone();
        }
    }
    centroid
}

fn calc_group_confidence(group: &[String]) -> f64 {
    if group.len() < 2 { return 1.0; }
    let mut sum = 0.0;
    let mut count = 0;
    for (i, x) in group.iter().enumerate() {
        for y in group.iter().skip(i + 1) {
            sum += jaro_winkler(x, y);
            count += 1;
        }
    }
    if count > 0 { sum  / (count as f64) } else { 1.0 }
}
