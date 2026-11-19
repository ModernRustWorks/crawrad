---
id: components-progress-bar-progress-bar
kind: doc
target_file: src/components/progress_bar.rs
line_hint: 4
author: 
created: 
title: "progress_bar"
tags: []
signature: "pub fn progress_bar () -> ComponentDesc"
---

`ProgressBar`: visual progress indicator for a long-running operation.

# Examples

```rust
use crawrad_registry::progress_bar;

let desc = progress_bar();
assert_eq!(desc.class, "ProgressBar");
```
