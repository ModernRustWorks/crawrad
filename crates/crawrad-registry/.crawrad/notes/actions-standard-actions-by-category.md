---
id: actions-standard-actions-by-category
kind: doc
target_file: src/actions.rs
line_hint: 282
author: 
created: 
title: "standard_actions_by_category"
tags: []
signature: "pub fn standard_actions_by_category () -> Vec < (& 'static str , Vec < StandardAction >) >"
---

Templates grouped by category in canonical order.

# Examples

```rust
use crawrad_registry::standard_actions_by_category;

let grouped = standard_actions_by_category();
assert!(!grouped.is_empty());
```
