---
id: web-run-webrun-serving
kind: doc
target_file: src/web_run.rs
line_hint: 1
author: 
created: 
title: "serving"
tags: []
signature: "pub fn serving (& self) -> bool"
---

Whether a page is being served: the IDE's Stop is enabled while it is.

# Examples

```rust
use crawrad_project::web_run::WebRun;

assert!(!WebRun::default().serving());
```
