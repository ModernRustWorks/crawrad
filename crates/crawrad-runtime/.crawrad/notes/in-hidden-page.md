---
id: in-hidden-page
kind: doc
target_file: src/lib.rs
line_hint: 256
author: 
created: 
title: "in_hidden_page"
tags: []
signature: "pub fn in_hidden_page (doc : & Document , id : & str) -> bool"
---

Checks if the node is inside an inactive page.

Is the node inside a page that is **not** the active one (or is it the page itself)?

# Examples

```rust
use crawrad_ir::Document;
use crawrad_runtime::in_hidden_page;

let doc = Document::new("form", "Form");
assert!(!in_hidden_page(&doc, "form"));
```
