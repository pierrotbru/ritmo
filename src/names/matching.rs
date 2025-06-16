// src/name_matching/matching.rs

use strsim::jaro_winkler;
use std::collections::HashSet;
use super::models::{MatchResult, NameMatch, MatchType, ParsedName};

impl super::manager::NameManager { // Implementa metodi su NameManager
    pub fn find_matches(&self, input_name: &str) -> MatchResult {
        let parsed_input_res = self.name_utils.parse_name(input_name);
        if parsed_input_res.is_err() {
            return MatchResult::NoMatch;
        }

        let parsed_input = parsed_input_res.unwrap();
        let normalized_input = self.name_utils.normalize_parsed_name_for_matching(&parsed_input);

        let mut candidate_ids: HashSet<i64> = HashSet::new();

        if let Some(ids) = self.normalized_key_index.get(&normalized_input) {
            candidate_ids.extend(ids);
        }

        for (normalized_key, ids) in &self.normalized_key_index {
            let levenshtein_sim = self.name_utils.normalized_levenshtein_distance(&normalized_input, normalized_key);
            if levenshtein_sim >= self.typo_threshold {
                candidate_ids.extend(ids);
            }
        }

        if candidate_ids.is_empty() {
            return MatchResult::NoMatch;
        }

        let mut matches = Vec::new();

        for &person_id in &candidate_ids {
            if let Some(person) = self.all_person_records.get(&person_id) {
                let mut best_match: Option<NameMatch> = None;
                let mut best_score = 0.0;

                let direct_score = jaro_winkler(&normalized_input, &person.normalized_key);
                if direct_score > best_score {
                    best_score = direct_score;
                    let match_type = if direct_score >= 0.99 {
                        MatchType::Exact
                    } else if direct_score >= self.typo_threshold {
                        let levenshtein_sim = self.name_utils.normalized_levenshtein_distance(&normalized_input, &person.normalized_key);
                        if levenshtein_sim >= 0.9 {
                            MatchType::TypoMinor
                        } else {
                            MatchType::TypoMajor
                        }
                    } else {
                        MatchType::Typo
                    };

                    best_match = Some(NameMatch {
                        person_id: person.id,
                        matched_name: person.parsed_name.display_name.clone(),
                        similarity_score: direct_score,
                        match_type,
                        confidence: direct_score * person.confidence,
                    });
                }

                // Learned Variants
                if best_score < 1.0 {
                    if let Some(learned_variant) = self.ml_learner.find_learned_variant(&normalized_input, &person.normalized_key) {
                        best_score = learned_variant.confidence.max(0.88);
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: learned_variant.confidence,
                            match_type: MatchType::Learned,
                            confidence: learned_variant.confidence * person.confidence,
                        });
                    }
                }

                // Known Variants
                if best_score < 1.0 {
                    if self.name_utils.are_known_variants(&normalized_input, &person.normalized_key) {
                        best_score = 0.95;
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: 0.95,
                            match_type: MatchType::Alias,
                            confidence: 0.95 * person.confidence,
                        });
                    }
                }

                // Name Order Swap
                if best_score < 1.0 {
                    let person_swapped_parsed_name = ParsedName {
                        given_name: person.parsed_name.surname.clone(),
                        surname: person.parsed_name.given_name.clone(),
                        middle_names: person.parsed_name.middle_names.clone(),
                        title: person.parsed_name.title.clone(),
                        suffix: person.parsed_name.suffix.clone(),
                        display_name: format!("{} {}", person.parsed_name.surname, person.parsed_name.given_name),
                    };
                    let person_swapped_normalized_key = self.name_utils.normalize_parsed_name_for_matching(&person_swapped_parsed_name);
                    let swap_score = jaro_winkler(&normalized_input, &person_swapped_normalized_key);

                    if swap_score > best_score && swap_score >= self.similarity_threshold {
                        best_score = swap_score;
                        best_match = Some(NameMatch {
                            person_id: person.id,
                            matched_name: person.parsed_name.display_name.clone(),
                            similarity_score: swap_score,
                            match_type: MatchType::NameOrder,
                            confidence: swap_score * person.confidence,
                        });
                    }
                }

                // Alias Matching
                if best_score < 1.0 {
                    for alias in &person.aliases {
                        let alias_score = jaro_winkler(&normalized_input, &self.name_utils.normalize_string(alias));
                        if alias_score > best_score {
                            best_score = alias_score;
                            best_match = Some(NameMatch {
                                person_id: person.id,
                                matched_name: alias.clone(),
                                similarity_score: alias_score,
                                match_type: MatchType::Alias,
                                confidence: alias_score * person.confidence * 0.9,
                            });
                        }
                    }
                }

                if let Some(m) = best_match {
                    if m.similarity_score >= self.similarity_threshold {
                        matches.push(m);
                    }
                }
            }
        }

        matches.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        if let Some(perfect_match) = matches.iter().find(|m| m.similarity_score >= 0.99) {
            return MatchResult::ExactMatch(perfect_match.person_id);
        }

        match matches.len() {
            0 => MatchResult::NoMatch,
            _ => {
                let top_match = &matches[0];
                if top_match.confidence > 0.9 {
                    MatchResult::HighConfidenceMatch(matches.into_iter().take(3).collect())
                } else if top_match.confidence > 0.75 {
                    MatchResult::PossibleMatches(matches.into_iter().take(5).collect())
                } else {
                    MatchResult::NoMatch
                }
            }
        }
    }
}
