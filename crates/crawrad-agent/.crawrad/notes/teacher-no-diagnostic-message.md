---
id: teacher-no-diagnostic-message
kind: doc
target_file: src/teacher.rs
line_hint: 231
author: 
created: 
title: "no_diagnostic_message"
tags: []
signature: "pub fn no_diagnostic_message () -> & 'static str"
---

Notice shown when caret line has no compiler diagnostic.

# Examples

```rust
use crawrad_agent::teacher::no_diagnostic_message;

assert!(no_diagnostic_message().contains("No compiler diagnostic"));
```
