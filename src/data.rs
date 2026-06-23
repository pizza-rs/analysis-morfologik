//! Lazy, config-overridable dictionary data for the Morfologik plugin.
//!
//! Each table is parsed **once** on first use — from the external
//! `config/analysis/morfologik/<file>` when a dictionary directory is
//! configured and the file exists, or from the embedded copy otherwise — and
//! then cached for the lifetime of the process. Filters capture a `&'static`
//! reference at construction time, so the per-token hot path performs no cell
//! access and no allocation beyond what the algorithm itself produces.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;

use hashbrown::HashSet;
use pizza_engine::analysis::OnceCellSync;

const NAMESPACE: &str = "morfologik";

#[cfg(feature = "embed-fallback")]
const EMBEDDED_STOPWORDS_UK: &str = include_str!("../data/stopwords_uk.txt");
#[cfg(not(feature = "embed-fallback"))]
const EMBEDDED_STOPWORDS_UK: &str = "";
#[cfg(feature = "embed-fallback")]
const EMBEDDED_POLISH_RULES: &str = include_str!("../data/polish_lemma_rules.txt");
#[cfg(not(feature = "embed-fallback"))]
const EMBEDDED_POLISH_RULES: &str = "";
#[cfg(feature = "embed-fallback")]
const EMBEDDED_UKRAINIAN_RULES: &str = include_str!("../data/ukrainian_lemma_rules.txt");
#[cfg(not(feature = "embed-fallback"))]
const EMBEDDED_UKRAINIAN_RULES: &str = "";

/// Read a dictionary file, preferring the external config copy (std only) and
/// falling back to the embedded text. Never fails: the embedded copy is always
/// present, so a missing or unreadable external file transparently degrades to
/// the bundled default.
fn load_text(file: &str, embedded: &'static str) -> Cow<'static, str> {
    #[cfg(feature = "std")]
    {
        pizza_engine::analysis::dict::load_str(NAMESPACE, file, Some(embedded))
            .unwrap_or(Cow::Borrowed(embedded))
    }
    #[cfg(not(feature = "std"))]
    {
        let _ = file;
        Cow::Borrowed(embedded)
    }
}

/// Parse `suffix<TAB>replacement` lines, preserving order (greedy longest-first
/// matching relies on the source ordering). Blank lines and `#` comments are
/// ignored.
fn parse_rules(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(|line| {
            let line = line.trim_end_matches('\r');
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (suffix, replacement) = line.split_once('\t')?;
            Some((String::from(suffix), String::from(replacement)))
        })
        .collect()
}

/// Ukrainian stop words, parsed once. Membership is an O(1) hash-set lookup.
pub(crate) fn ukrainian_stopwords() -> &'static HashSet<String> {
    static CELL: OnceCellSync<HashSet<String>> = OnceCellSync::new();
    CELL.get_or_init(|| {
        load_text("stopwords_uk.txt", EMBEDDED_STOPWORDS_UK)
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(String::from)
            .collect()
    })
}

/// Polish lemmatization rules (suffix → replacement), in source order.
pub(crate) fn polish_lemma_rules() -> &'static [(String, String)] {
    static CELL: OnceCellSync<Vec<(String, String)>> = OnceCellSync::new();
    CELL.get_or_init(|| {
        let text = load_text("polish_lemma_rules.txt", EMBEDDED_POLISH_RULES);
        parse_rules(&text)
    })
    .as_slice()
}

/// Ukrainian lemmatization rules (suffix → replacement), in source order.
pub(crate) fn ukrainian_lemma_rules() -> &'static [(String, String)] {
    static CELL: OnceCellSync<Vec<(String, String)>> = OnceCellSync::new();
    CELL.get_or_init(|| {
        let text = load_text("ukrainian_lemma_rules.txt", EMBEDDED_UKRAINIAN_RULES);
        parse_rules(&text)
    })
    .as_slice()
}
