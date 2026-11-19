---
id: web-run-webrun-stop
kind: doc
target_file: src/web_run.rs
line_hint: 1
author: 
created: 
title: "stop"
tags: []
signature: "pub fn stop (& mut self) -> bool"
---

Stops the served page; `false` when none was up.

# Examples

```rust
use crawrad_project::web_run::WebRun;

let mut run = WebRun::default();
assert!(!run.stop());
```
