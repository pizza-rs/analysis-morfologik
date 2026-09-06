//! Polish Morfologik lemmatizer.
//!
//! Provides lemmatization by applying morphological suffix rules derived from
//! the Morfologik Polish dictionary (4.8M word->stem pairs, morfologik-polish 2.1.9).
//! Rules are extracted from the real dictionary data, keeping patterns with 100+ occurrences.
//! This gives 819 rules covering ~2.86M word->stem pairs (59.5% coverage).
//!
//! Rules are ordered by suffix length (longest first) for greedy matching.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

use crate::data::polish_lemma_rules;

/// Polish morphological lemmatizer filter.
///
/// Reduces inflected Polish words to their base/lemma form using suffix-based
/// rules derived from the Morfologik Polish dictionary. The rule table is loaded
/// once from `config/analysis/morfologik/polish_lemma_rules.txt` (or the embedded
/// copy) and shared as `&'static`, so the per-token path allocates only the
/// lemma it returns.
///
/// Equivalent to Elasticsearch's `morfologik_stem` filter.
#[derive(Clone, Debug)]
pub struct MorfologikFilter {
    rules: &'static [(String, String)],
}

impl Default for MorfologikFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl MorfologikFilter {
    pub fn new() -> Self {
        Self {
            rules: polish_lemma_rules(),
        }
    }

    /// Attempt to lemmatize a Polish word.
    fn lemmatize(&self, word: &str) -> Option<String> {
        let lower = word.to_lowercase();

        if lower.len() < 4 {
            return None;
        }

        for (suffix, replacement) in self.rules.iter() {
            if lower.ends_with(suffix.as_str()) {
                let stem_len = lower.len() - suffix.len();
                if stem_len >= 2 {
                    let mut lemma = String::from(&lower[..stem_len]);
                    lemma.push_str(replacement);
                    return Some(lemma);
                }
            }
        }

        None
    }
}

impl TokenFilter for MorfologikFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if let Some(lemma) = self.lemmatize(term) {
            if lemma != term.to_lowercase() {
                token.term = Cow::Owned(lemma);
            }
        }
        (true, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lemmatization() {
        let f = MorfologikFilter::new();
        // Noun: instrumental plural -> nominative
        assert_eq!(f.lemmatize("domami"), Some("dom".to_string()));
    }

    #[test]
    fn test_short_words_unchanged() {
        let f = MorfologikFilter::new();
        assert_eq!(f.lemmatize("do"), None);
        assert_eq!(f.lemmatize("na"), None);
    }
}
