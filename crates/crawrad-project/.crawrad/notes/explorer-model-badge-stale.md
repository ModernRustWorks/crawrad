---
id: explorer-model-badge-stale
kind: doc
target_file: src/explorer_model.rs
line_hint: 314
author: 
created: 
title: "BADGE_STALE"
tags: []
signature: "pub const BADGE_STALE : & str = \"stale\""
---

Badge for a row with no owner to assert (an orphaned designer file).

The badge for the row that **has no owner to assert** — the orphaned `*.designer.rs`.

A constant, and not the word written in every place that uses it: it appears in
the line, in the legend, and in both shells, and written four times is the legend
explaining a badge that the screen stopped showing.

# Examples

```rust
use crawrad_project::explorer_model::BADGE_STALE;

assert_eq!(BADGE_STALE, "stale");
```
