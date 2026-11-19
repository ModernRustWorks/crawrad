---
id: teacher-fix-explain-prompt
kind: doc
target_file: src/teacher.rs
line_hint: 131
author: 
created: 
title: "fix_explain_prompt"
tags: []
signature: "pub fn fix_explain_prompt (g : & Grounding , lang : & str) -> String"
---

Prompt for fixing a diagnostic with minimal diff and explaining why.

# Examples

```rust
use crawrad_agent::teacher::{Grounding, fix_explain_prompt};

let g = Grounding {
    code: Some("E0382".to_string()),
    message: "use of moved value".to_string(),
    file: "main.rs".to_string(),
    line: 10,
    snippet: "let y = x;".to_string(),
    explain: None,
};
let prompt = fix_explain_prompt(&g, "English");
assert!(prompt.contains("[crawrad-teacher]"));
```
