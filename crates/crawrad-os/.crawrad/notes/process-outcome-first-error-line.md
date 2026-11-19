---
id: process-outcome-first-error-line
kind: doc
target_file: src/process.rs
line_hint: 160
author: 
created: 
title: "Outcome::first_error_line"
tags: []
signature: "pub fn first_error_line (& self) -> Option < & str >"
---

The first line cargo marked as an error, if it marked one.

# Examples

```rust
use crawrad_os::process::Outcome;

let outcome = Outcome {
    stderr: "Compiling\nerror: aborting due to previous error\n".to_string(),
    ..Default::default()
};
assert_eq!(outcome.first_error_line(), Some("error: aborting due to previous error"));
```
