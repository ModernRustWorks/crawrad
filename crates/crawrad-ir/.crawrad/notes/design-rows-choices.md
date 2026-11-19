---
id: design-rows-choices
kind: doc
target_file: src/design_rows.rs
line_hint: 15
author: 
created: 
title: "CHOICES"
tags: []
signature: "pub const CHOICES: [&str; 4] = [\"Off\", \"8\", \"20\", \"50\"]"
---

The choices the Inspector offers for [`DESIGN_ROWS_KEY`].

# Examples

```rust
use crawrad_ir::design_rows::CHOICES;

assert_eq!(CHOICES[0], "Off");
```
