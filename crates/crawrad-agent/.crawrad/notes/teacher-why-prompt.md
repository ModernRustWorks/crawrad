---
id: teacher-why-prompt
kind: doc
target_file: src/teacher.rs
line_hint: 166
author: 
created: 
title: "why_prompt"
tags: []
signature: "pub fn why_prompt (g : & Grounding , lang : & str) -> String"
---

Prompt for explaining why the compiler rejects code on caret's line.

Prompt for explaining why the compiler rejects code on caret's line without modifying it.

# Examples

```rust
use crawrad_agent::teacher::{Grounding, why_prompt};

let g = Grounding {
    code: Some("E0382".to_string()),
    message: "use of moved value".to_string(),
    file: "main.rs".to_string(),
    line: 10,
    snippet: "let y = x;".to_string(),
    explain: None,
};
let prompt = why_prompt(&g, "English");
assert!(prompt.contains("[crawrad-teacher]"));
```
