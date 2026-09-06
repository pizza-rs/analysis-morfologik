//! Ukrainian stemmer / lemmatizer.
//!
//! Provides morphological stemming for Ukrainian text using suffix rules derived from
//! the Morfologik Ukrainian dictionary (3.5M word->stem pairs, morfologik-ukrainian-search 4.9.1).
//! Rules are extracted from the real dictionary data, keeping patterns with 100+ occurrences.
//! This gives 452 rules covering ~1.76M word->stem pairs (50.5% coverage).
//!
//! Rules are ordered by suffix length (longest first) for greedy matching.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

use crate::data::ukrainian_lemma_rules;

/// Ukrainian stemming token filter.
///
/// Reduces inflected Ukrainian words to their base/lemma form using suffix-based
/// rules derived from the Morfologik Ukrainian dictionary. The rule table is
/// loaded once from `config/analysis/morfologik/ukrainian_lemma_rules.txt` (or
/// the embedded copy) and shared as `&'static`.
///
/// Equivalent to Elasticsearch's Ukrainian morphology analysis plugin.
#[derive(Clone, Debug)]
pub struct UkrainianStemFilter {
    rules: &'static [(String, String)],
}

impl Default for UkrainianStemFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl UkrainianStemFilter {
    pub fn new() -> Self {
        Self {
            rules: ukrainian_lemma_rules(),
        }
    }

    fn stem(&self, word: &str) -> Option<String> {
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

impl TokenFilter for UkrainianStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if let Some(stemmed) = self.stem(term) {
            if stemmed != term {
                token.term = Cow::Owned(stemmed);
            }
        }
        (true, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_stemming() {
        let f = UkrainianStemFilter::new();
        // Test instrumental plural
        assert!(f.stem("домами").is_some());
    }

    #[test]
    fn test_short_words() {
        let f = UkrainianStemFilter::new();
        assert_eq!(f.stem("він"), None);
    }
}
