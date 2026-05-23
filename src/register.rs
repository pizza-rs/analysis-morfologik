//! Register Morfologik analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;

use pizza_engine::analysis::AnalysisFactory;

use crate::{MorfologikFilter, UkrainianStemFilter, UkrainianStopFilter};

/// Register Morfologik token filters.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("morfologik_stem", Box::new(MorfologikFilter::new()));
    factory.register_token_filter("ukrainian_stem", Box::new(UkrainianStemFilter::new()));
    factory.register_token_filter("ukrainian_stop", Box::new(UkrainianStopFilter::new()));
}
