//! Ukrainian stop words filter.
//!
//! Stop words list sourced from Apache Lucene's Ukrainian analyzer
//! (licensed under Apache 2.0).

use alloc::borrow::Cow;

use pizza_engine::analysis::{Token, TokenFilter};

/// Ukrainian stop words (1269 entries from Lucene).
pub static UKRAINIAN_STOP_WORDS: &str = include_str!("../data/stopwords_uk.txt");

/// Ukrainian stop words filter.
///
/// Removes common Ukrainian function words that carry little semantic value.
/// The stop word list is sourced from Apache Lucene's `UkrainianMorfologikAnalyzer`.
#[derive(Clone, Debug)]
pub struct UkrainianStopFilter {
    _priv: (),
}

impl Default for UkrainianStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl UkrainianStopFilter {
    pub fn new() -> Self {
        Self { _priv: () }
    }

    fn is_stop_word(&self, word: &str) -> bool {
        let lower = word.to_lowercase();
        UKRAINIAN_STOP_WORDS.lines().any(|w| w == lower)
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
