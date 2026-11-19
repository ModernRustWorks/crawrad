---
id: teacher-grounding
kind: doc
target_file: src/teacher.rs
line_hint: 112
author: 
created: 
title: "Grounding"
tags: []
signature: "pub struct Grounding"
---

Grounding context for diagnostic explanation and fixing.

# Examples

```rust
use crawrad_agent::teacher::Grounding;

let g = Grounding {
    code: Some("E0382".to_string()),
    message: "use of moved value".to_string(),
    file: "main.rs".to_string(),
    line: 10,
    snippet: "println!(\"{}\", x);".to_string(),
    explain: None,
};
assert_eq!(g.line, 10);
```
