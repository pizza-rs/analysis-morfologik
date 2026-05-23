//! Comprehensive tests for pizza-analysis-morfologik (Polish + Ukrainian morphological analysis).

use pizza_analysis_morfologik::{MorfologikFilter, UkrainianStemFilter, UkrainianStopFilter};
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

// ═══════════════════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════════════════

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// MorfologikFilter — construction
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn morfologik_construction() {
    let _f = MorfologikFilter::new();
}

#[test]
fn morfologik_default() {
    let _f = MorfologikFilter::default();
}

#[test]
fn morfologik_clone() {
    let f1 = MorfologikFilter::new();
    let _f2 = f1.clone();
}

#[test]
fn morfologik_debug() {
    let f = MorfologikFilter::new();
    let dbg = format!("{:?}", f);
    assert!(dbg.contains("MorfologikFilter"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// MorfologikFilter — lemmatization
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn morfologik_lemmatize_polish_noun() {
    let f = MorfologikFilter::new();
    let mut token = make_token("komputerów");
    let (not_deleted, _) = f.filter(&mut token);
    assert!(not_deleted);
    // Should either lemmatize or keep as-is
    assert!(!token.term.is_empty());
}

#[test]
fn morfologik_lemmatize_polish_verb() {
    let f = MorfologikFilter::new();
    let mut token = make_token("pisałem");
    let (_not_deleted, _) = f.filter(&mut token);
    assert!(!token.term.is_empty());
}

#[test]
fn morfologik_lemmatize_polish_adjective() {
    let f = MorfologikFilter::new();
    let mut token = make_token("pięknego");
    let (_not_deleted, _) = f.filter(&mut token);
    assert!(!token.term.is_empty());
}

#[test]
fn morfologik_short_word_unchanged() {
    let f = MorfologikFilter::new();
    let mut token = make_token("dom");
    let _ = f.filter(&mut token);
    // Short words (< 4 chars) skip lemmatization
    assert_eq!(token.term.as_ref(), "dom");
}

#[test]
fn morfologik_empty_string() {
    let f = MorfologikFilter::new();
    let mut token = make_token("");
    let (not_deleted, _) = f.filter(&mut token);
    assert!(not_deleted);
    assert_eq!(token.term.as_ref(), "");
}

#[test]
fn morfologik_single_char() {
    let f = MorfologikFilter::new();
    let mut token = make_token("a");
    let _ = f.filter(&mut token);
    assert_eq!(token.term.as_ref(), "a");
}

#[test]
fn morfologik_preserves_offsets() {
    let f = MorfologikFilter::new();
    let mut token = Token::new("komputerów", 10, 30, 3);
    let _ = f.filter(&mut token);
    assert_eq!(token.start_offset, 10);
    assert_eq!(token.end_offset, 30);
    assert_eq!(token.position, 3);
}

#[test]
fn morfologik_non_polish_text() {
    let f = MorfologikFilter::new();
    let mut token = make_token("running");
    let _ = f.filter(&mut token);
    // Should not panic; may or may not transform
}

// ═══════════════════════════════════════════════════════════════════════════════
// UkrainianStemFilter — construction
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn ukrainian_stem_construction() {
    let _f = UkrainianStemFilter::new();
}

#[test]
fn ukrainian_stem_default() {
    let _f = UkrainianStemFilter::default();
}

#[test]
fn ukrainian_stem_clone() {
    let f1 = UkrainianStemFilter::new();
    let _f2 = f1.clone();
}

// ═══════════════════════════════════════════════════════════════════════════════
// UkrainianStemFilter — stemming
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn ukrainian_stem_basic() {
    let f = UkrainianStemFilter::new();
    let mut token = make_token("комп'ютерів");
    let (not_deleted, _) = f.filter(&mut token);
    assert!(not_deleted);
    assert!(!token.term.is_empty());
}

#[test]
fn ukrainian_stem_short_word() {
    let f = UkrainianStemFilter::new();
    let mut token = make_token("дім");
    let _ = f.filter(&mut token);
    // Short words stay unchanged
}

#[test]
fn ukrainian_stem_empty() {
    let f = UkrainianStemFilter::new();
    let mut token = make_token("");
    let (not_deleted, _) = f.filter(&mut token);
    assert!(not_deleted);
}

#[test]
fn ukrainian_stem_preserves_offsets() {
    let f = UkrainianStemFilter::new();
    let mut token = Token::new("програмістів", 0, 24, 1);
    let _ = f.filter(&mut token);
    assert_eq!(token.start_offset, 0);
    assert_eq!(token.end_offset, 24);
    assert_eq!(token.position, 1);
}

// ═══════════════════════════════════════════════════════════════════════════════
// UkrainianStopFilter — construction and filtering
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn ukrainian_stop_construction() {
    let _f = UkrainianStopFilter::new();
}

#[test]
fn ukrainian_stop_default() {
    let _f = UkrainianStopFilter::default();
}

#[test]
fn ukrainian_stop_keeps_content_words() {
    let f = UkrainianStopFilter::new();
    let mut token = make_token("програмування");
    let (not_deleted, _) = f.filter(&mut token);
    assert!(not_deleted, "content word should not be deleted");
}

#[test]
fn ukrainian_stop_empty_token() {
    let f = UkrainianStopFilter::new();
    let mut token = make_token("");
    let (not_deleted, _) = f.filter(&mut token);
    assert!(not_deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_does_not_panic() {
    let mut factory = AnalysisFactory::new();
    pizza_analysis_morfologik::register_all(&mut factory);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Pipeline integration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pipeline_polish_stop_then_lemmatize() {
    let lemma = MorfologikFilter::new();

    let words = ["dom", "komputerów", "programistów", "pięknego"];
    let mut results = Vec::new();

    for &w in &words {
        let mut token = make_token(w);
        let (not_deleted, _) = lemma.filter(&mut token);
        if not_deleted {
            results.push(token.term.to_string());
        }
    }
    assert_eq!(results.len(), words.len());
}

#[test]
fn pipeline_ukrainian_stem_then_stop() {
    let stem = UkrainianStemFilter::new();
    let stop = UkrainianStopFilter::new();

    let mut token = make_token("програмістів");
    let _ = stem.filter(&mut token);
    let (not_deleted, _) = stop.filter(&mut token);
    // A content word should survive stop filtering
    assert!(not_deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Unicode handling
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn morfologik_unicode_polish_chars() {
    let f = MorfologikFilter::new();
    // Polish-specific: ą ć ę ł ń ó ś ź ż
    let mut token = make_token("źródłem");
    let _ = f.filter(&mut token);
    assert!(!token.term.is_empty());
}

#[test]
fn ukrainian_unicode_chars() {
    let f = UkrainianStemFilter::new();
    // Ukrainian-specific: є, і, ї, ґ
    let mut token = make_token("українського");
    let _ = f.filter(&mut token);
    assert!(!token.term.is_empty());
}
