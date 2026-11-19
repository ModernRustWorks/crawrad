---
id: web-run-webrun-pump
kind: doc
target_file: src/web_run.rs
line_hint: 1
author: 
created: 
title: "pump"
tags: []
signature: "pub fn pump (& mut self) -> Option < (String , Option < String >) >"
---

Drains the packaging: the status line and the URL to open, once it ends.

# Examples

```rust
use crawrad_project::web_run::WebRun;

let mut run = WebRun::default();
assert_eq!(run.pump(), None);
```
