---
id: explorer-rows-rows-section-section
kind: doc
target_file: src/explorer_rows/rows/section.rs
line_hint: 30
author: 
created: 
title: "Section"
tags: []
signature: "pub struct Section"
---

A foldable section header.

# Examples

```rust
use crawrad_project::explorer_rows::rows::section::Section;
use crawrad_project::explorer_rows::row::ExplorerRow;

let s = Section::new("Legend", false, "explorer.toggle_legend");
assert_eq!(s.text(), "Legend");
```
