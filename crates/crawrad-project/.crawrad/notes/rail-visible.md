---
id: rail-visible
kind: doc
target_file: src/rail.rs
line_hint: 65
author: 
created: 
title: "visible"
tags: []
signature: "pub fn visible (has_form: bool) -> Vec < Panel >"
---

The rail panels visible depending on whether the project has a form.

When `has_form` is false, `Panel::Inspector` is omitted from the rail,
preserving the order of all other panels. When true, it returns `RAIL`.

# Examples

```rust
use crawrad_actions::Panel;
use crawrad_project::rail::{visible, RAIL};

assert_eq!(visible(true), RAIL);
assert!(!visible(false).contains(&Panel::Inspector));
```
