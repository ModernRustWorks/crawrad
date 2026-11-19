---
id: options-option-rows
kind: doc
target_file: src/options.rs
line_hint: 421
author: 
created: 
title: "option_rows"
tags: []
signature: "pub fn option_rows (is_closed : & dyn Fn (& str) -> bool) -> Vec < OptionRow >"
---

The flattened tree, with closed groups hiding their children.

# Examples

```rust
use crawrad_project::options::option_rows;

let rows = option_rows(&|_| false);
assert!(!rows.is_empty());
```
