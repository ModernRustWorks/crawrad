---
id: explorer-rows-icon-warning
kind: doc
target_file: src/explorer_rows/icon.rs
line_hint: 71
author: 
created: 
title: "WARNING"
tags: []
signature: "pub const WARNING : RowIcon = \"\\u{ea6c}\""
---

Something is wrong and somebody has to do something about it.

Codicons `warning`.

One constant for the leftover `*.designer.rs` and for the sentence that
warns, because they are the same drawing — and two names for one drawing is
how a set starts to drift.

# Examples

```rust
use crawrad_project::explorer_rows::icon::WARNING;

assert!(!WARNING.is_empty());
```
