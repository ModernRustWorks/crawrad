---
id: templates-template
kind: doc
target_file: src/templates.rs
line_hint: 43
author: 
created: 
title: "Template"
tags: []
signature: "pub struct Template"
---

A starting point offered on the welcome screen.

# Examples

```rust
use crawrad_project::templates::Template;

let t = Template::default_template();
assert_eq!(t.name, "Blank form");
```
