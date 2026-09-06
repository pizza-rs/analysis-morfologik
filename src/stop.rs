//! Ukrainian stop words filter.
//!
//! Stop words list sourced from Apache Lucene's Ukrainian analyzer
//! (licensed under Apache 2.0).

use alloc::string::String;
use alloc::vec::Vec;

use hashbrown::HashSet;
use pizza_engine::analysis::Token;
use pizza_engine::analysis::TokenFilter;

use crate::data::ukrainian_stopwords;

/// Ukrainian stop words filter.
///
/// Removes common Ukrainian function words that carry little semantic value.
/// The list is loaded once from `config/analysis/morfologik/stopwords_uk.txt`
/// (or the embedded Apache Lucene copy) into a shared `&'static` hash set, so
/// membership is O(1) and the hot path allocates only the lowercased term.
#[derive(Clone, Debug)]
pub struct UkrainianStopFilter {
    stop: &'static HashSet<String>,
}

impl Default for UkrainianStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl UkrainianStopFilter {
    pub fn new() -> Self {
        Self {
            stop: ukrainian_stopwords(),
        }
    }

    fn is_stop_word(&self, word: &str) -> bool {
        self.stop.contains(&word.to_lowercase())
    }
}

impl TokenFilter for UkrainianStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let term = token.term.as_ref();
        if self.is_stop_word(term) {
            (false, None)
        } else {
            (true, None)
        }
    }
}
