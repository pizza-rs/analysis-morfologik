# pizza-analysis-morfologik

Polish and Ukrainian morphological analysis for the [Pizza](https://pizza.rs) search engine. Provides lemmatization and stemming using suffix rules derived from the real [Morfologik](https://github.com/morfologik/morfologik-stemming) dictionaries.

## Components

| Name | Type | Description |
|------|------|-------------|
| `morfologik_stem` | Token Filter | Polish lemmatizer — reduces inflected words to base/lemma form |
| `ukrainian_stem` | Token Filter | Ukrainian stemmer — suffix-based stemming |
| `ukrainian_stop` | Token Filter | Ukrainian stop words removal (1,269 words) |

## Usage

### Polish Lemmatization

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["lowercase", "morfologik_stem"]
  }
}
```

### Ukrainian Analysis

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["lowercase", "ukrainian_stop", "ukrainian_stem"]
  }
}
```

### Examples

**Polish:**

| Input | Lemmatized |
|-------|-----------|
| `domami` | `dom` (house) |
| `komputerów` | `komputer` (computer) |
| `pisalibyście` | `pisać` (to write) |
| `polskich` | `polski` (Polish) |

**Ukrainian:**

| Input | Stemmed |
|-------|---------|
| `будинків` | stemmed form |
| `працювали` | `працювати` (to work) |
| `домами` | stemmed form |

## Algorithm

Uses greedy longest-suffix-first matching against a pre-computed rule table:

1. Convert input to lowercase
2. Iterate rules ordered by suffix length (longest first)
3. If the word ends with the rule's suffix and the remaining stem is ≥2 characters, apply the replacement
4. Return the first match (greedy)

This approach sacrifices some accuracy compared to a full FSA dictionary lookup, but is fast, compact, and covers the most common morphological patterns.

## Data Sources

Rules are extracted from the actual Morfologik dictionaries using a frequency-based approach:

- **Polish**: 819 rules derived from `morfologik-polish` 2.1.9 (4.8M word→stem pairs). Rules cover patterns occurring in 100+ words, giving coverage of ~2.86M pairs (59.5%).
- **Ukrainian**: 452 rules derived from `morfologik-ukrainian-search` 4.9.1 (3.5M word→stem pairs). Rules cover patterns occurring in 100+ words, giving coverage of ~1.76M pairs (50.5%).
- **Ukrainian stop words**: 1,269 entries from Apache Lucene's `UkrainianMorfologikAnalyzer`

### Why Not the Full FSA Dictionary?

The original Morfologik uses CFSA2 (Compact Finite State Automaton) binary dictionaries (~2.8MB Polish, ~7.2MB Ukrainian). These contain 4.8M+ entries but require implementing a complex FSA reader. The suffix-rule approach provides:

- Compact size (~40KB vs ~10MB for full dictionaries)
- No external binary data dependencies
- Fast matching (simple string suffix comparison)
- Covers the majority of real-world inflected forms

## Features

- `polish` (default) — Enable Polish lemmatization rules
- `ukrainian` — Enable Ukrainian stemming rules

## License

Apache-2.0 (rules derived from Morfologik, licensed under BSD)
