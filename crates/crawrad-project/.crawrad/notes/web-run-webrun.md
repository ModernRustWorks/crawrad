---
id: web-run-webrun
kind: doc
target_file: src/web_run.rs
line_hint: 1
author: 
created: 
title: "WebRun"
tags: []
signature: "pub struct WebRun"
---

The web Run of the IDE: packages in the background and serves the page.

# Examples

```rust
use crawrad_project::web_run::WebRun;

let mut run = WebRun::default();
assert!(run.pump().is_none());
```
