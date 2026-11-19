---
id: diagnostics-model-format-file-size
kind: doc
target_file: src/diagnostics_model.rs
line_hint: 260
author: 
created: 
title: "format_file_size"
tags: []
signature: "pub fn format_file_size (bytes : u64) -> String"
---

Formats byte counts to human readable strings.

# Examples

```rust
use crawrad_project::diagnostics_model::format_file_size;

assert_eq!(format_file_size(500), "500 B");
assert_eq!(format_file_size(2048), "2.0 KB");
```
