---
id: teacher-teach-prompt
kind: doc
target_file: src/teacher.rs
line_hint: 201
author: 
created: 
title: "teach_prompt"
tags: []
signature: "pub fn teach_prompt (focus : & str , lang : & str) -> String"
---

Prompt for teaching selected code line by line.

# Examples

```rust
use crawrad_agent::teacher::teach_prompt;

let prompt = teach_prompt("borrowing", "English");
assert!(prompt.contains("borrowing"));
```
