---
id: inspector-model-name-width-default
kind: doc
target_file: src/inspector_model.rs
line_hint: 609
author: 
created: 
title: "NAME_WIDTH_DEFAULT"
tags: []
signature: "pub const NAME_WIDTH_DEFAULT : f64 = 136.0"
---

The width the NAME column is born with — the splitter's rest (#554).

It holds `ShowColumnHeaders`, the widest label of the catalogue, and at the
dock's default width still leaves the values more than the old fold column,
so `[akLeft,akTop]` fits on the first open. The shell's
`dock_fits_widest_catalog_label` keeps it equal to `CRAWRAD_NAME_COLUMN`.

# Examples

```rust
use crawrad_actions::inspector_model::NAME_WIDTH_DEFAULT;

assert_eq!(NAME_WIDTH_DEFAULT, 136.0);
```
