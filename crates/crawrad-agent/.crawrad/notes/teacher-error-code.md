---
id: teacher-error-code
kind: doc
target_file: src/teacher.rs
line_hint: 6
author: 
created: 
title: "error_code"
tags: []
signature: "pub fn error_code (code : Option < & str >) -> Option < String >"
---

Normalizes a rustc diagnostic error code to `E` + 4 digits.

Normalizes a rustc diagnostic error code to `E0382` when it looks like
`E` + 4 digits (case-insensitive); returns `None` for lints or invalid formats.

# Examples

```rust
use crawrad_agent::teacher::error_code;

assert_eq!(error_code(Some("e0382")), Some("E0382".to_string()));
assert_eq!(error_code(Some("clippy::all")), None);
```
