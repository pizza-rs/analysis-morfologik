//! Register Morfologik analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;

use pizza_engine::analysis::AnalysisFactory;

use crate::{MorfologikFilter, UkrainianStemFilter, UkrainianStopFilter};

/// Register Morfologik token filters. Construction is lazy: each filter (and the
/// dictionary it loads) is built only on first use of that named filter.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter_with("morfologik_stem", || Box::new(MorfologikFilter::new()));
    factory.register_token_filter_with("ukrainian_stem", || Box::new(UkrainianStemFilter::new()));
    factory.register_token_filter_with("ukrainian_stop", || Box::new(UkrainianStopFilter::new()));
}
