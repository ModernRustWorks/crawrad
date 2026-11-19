---
id: web-run-index-html
kind: doc
target_file: src/web_run.rs
line_hint: 31
author: 
created: 
title: "index_html"
tags: []
signature: "pub fn index_html (wasm : & str) -> String"
---

Generates the host HTML page that bootstraps the Makepad WebAssembly app.

# Examples

```rust
use crawrad_project::web_run::index_html;

let html = index_html("demo");
assert!(html.contains("<title>demo</title>"));
```
