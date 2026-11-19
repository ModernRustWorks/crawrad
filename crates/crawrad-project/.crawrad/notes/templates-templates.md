---
id: templates-templates
kind: doc
target_file: src/templates.rs
line_hint: 60
author: 
created: 
title: "TEMPLATES"
tags: []
signature: "pub static TEMPLATES : & [Template] = & [Template { name : \"Blank form\" , description : \"an empty form, ready to design\" , size : FORM , caption : \"Form1\" , pieces : & [] , }]"
---

The templates, in the order they appear.

**The bar for expansion is strict** (ADR 0021): a template enters if it can
be described entirely as catalogue components at positions. A template that
requires a component that does not exist yet does not enter as a dead row —
it is a Phase 3 item.

`static`, and not `const`: a `const` is copied in EACH use, so
`TEMPLATES[0]` and the first item of `TEMPLATES.iter()` would end up at
different addresses and [`Template::default_template`] would cease to be an item of
this list to become something similar to it. The test `the_default_is_one_of_the_templates_in_the_list`
measured this — it failed with `const` and passed with `static`.

# Examples

```rust
use crawrad_project::templates::TEMPLATES;

assert!(!TEMPLATES.is_empty());
```
