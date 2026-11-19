---
id: options-optionssearchmatches
kind: doc
target_file: src/options.rs
line_hint: 442
author: 
created: 
title: "OptionsSearchMatches"
tags: []
signature: "pub struct OptionsSearchMatches"
---

Search match results for cards in the options search page.

# Examples

```rust
use crawrad_project::options::OptionsSearchMatches;

let matches = OptionsSearchMatches::default();
assert!(!matches.has_any());
```
