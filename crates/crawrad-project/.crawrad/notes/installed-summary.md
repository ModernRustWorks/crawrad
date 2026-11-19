---
id: installed-summary
kind: doc
target_file: src/installed.rs
line_hint: 92
author: 
created: 
title: "summary"
tags: []
signature: "pub fn summary () -> String"
---

Status bar summary of loaded and refused component packages.

What the IDE's catalogue loaded — and what it REFUSED: the status bar's
sentence for `Installed Packages…`.

# Examples

```rust
use crawrad_project::installed::summary;

assert!(summary().contains("package(s) installed"));
```
