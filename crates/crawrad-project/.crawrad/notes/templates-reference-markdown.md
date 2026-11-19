---
id: templates-reference-markdown
kind: doc
target_file: src/reference.rs
line_hint: 10
author: 
created: 
title: "templates_reference_markdown"
tags: []
signature: "pub fn templates_reference_markdown () -> String"
---

The manual page listing every project template.

# Examples

```rust
let md = crawrad_project::reference::templates_reference_markdown();
assert!(md.starts_with("# Project templates"));
```
