---
id: web-run-webrun-start
kind: doc
target_file: src/web_run.rs
line_hint: 1
author: 
created: 
title: "start"
tags: []
signature: "pub fn start (& mut self , root : & Path , package : & str) -> String"
---

Starts packaging `package` for the browser and answers the status line.

# Examples

```rust
use std::path::Path;
use crawrad_project::web_run::WebRun;

let mut run = WebRun::default();
assert!(run.start(Path::new("missing"), "demo").contains("demo"));
```
