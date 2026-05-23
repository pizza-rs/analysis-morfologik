<div align="center">

# 🇵🇱 pizza-analysis-morfologik

**Polish & Ukrainian dictionary-based stemming for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--morfologik-blue)](https://github.com/pizza-rs/analysis-morfologik)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Dictionary-based lemmatization for Polish and Ukrainian using the Morfologik
algorithm. Unlike algorithmic stemmers that apply suffix rules, Morfologik uses
a finite-state automaton (FSA) dictionary to look up the correct lemma for each
word form — providing higher accuracy for inflected Slavic languages.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `morfologik_stem` | Polish dictionary lemmatizer (FSA-based) |
| TokenFilter | `ukrainian_stem` | Ukrainian dictionary lemmatizer |
| TokenFilter | `ukrainian_stop` | Ukrainian stop words |

### Why Dictionary Stemming?

Polish has ~15 noun declension patterns and ~11 verb conjugation patterns.
Rule-based stemmers frequently under/over-stem. Morfologik looks up each token
in a compiled dictionary automaton to find the correct base form:

- `samochody` → `samochód` (cars → car)
- `biegającego` → `biegający` (running, genitive → running)

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_morfologik::register_all(&mut factory);

// Use as individual filters:
let stem = factory.get_token_filter("morfologik_stem").unwrap();
```

## Installation

```toml
[dependencies]
pizza-analysis-morfologik = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["morfologik"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
