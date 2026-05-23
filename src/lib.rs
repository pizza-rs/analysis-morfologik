#![cfg_attr(not(feature = "std"), no_std)]
//! Morfologik-based lemmatizer for Polish and Ukrainian.
//!
//! Provides morphological analysis and lemmatization using comprehensive
//! suffix-based rules covering noun declension, verb conjugation, and
//! adjective agreement for both Polish and Ukrainian languages.
//!
//! The rule tables contain ~200+ patterns per language, ordered by suffix
//! length (longest first) for greedy matching. Rules are categorized by
//! part of speech (verb → noun → adjective) matching typical disambiguation
//! priority.
//!
//! # Components
//!
//! - [`MorfologikFilter`] — Polish lemmatization token filter
//! - [`UkrainianStemFilter`] — Ukrainian stemming token filter
//! - [`UkrainianStopFilter`] — Ukrainian stop words filter
extern crate alloc;
mod polish;
mod stop;
mod ukrainian;

pub use polish::MorfologikFilter;
pub use stop::UkrainianStopFilter;
pub use ukrainian::UkrainianStemFilter;
pub mod register;
pub use register::register_all;
