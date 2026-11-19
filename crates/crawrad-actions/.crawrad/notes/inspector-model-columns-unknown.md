---
id: inspector-model-columns-unknown
kind: doc
target_file: src/inspector_model.rs
line_hint: 162
author: 
created: 
title: "COLUMNS_UNKNOWN"
tags: []
signature: "pub const COLUMNS_UNKNOWN : & str = \"Columns not listed\""
---

Heads the reason a Data > Field row stays free text.

When the dataset's columns cannot be read the row keeps the text box and
says this, then the reason (for example a missing `crawrad-dbprobe`).

# Examples

```rust
use crawrad_actions::inspector_model::COLUMNS_UNKNOWN;

assert_eq!(COLUMNS_UNKNOWN, "Columns not listed");
```
