---
id: teacher-compare-prompt
kind: doc
target_file: src/teacher.rs
line_hint: 217
author: 
created: 
title: "compare_prompt"
tags: []
signature: "pub fn compare_prompt (a : & str , b : & str , lang : & str) -> String"
---

Prompt for comparing two Rust choices.

# Examples

```rust
use crawrad_agent::teacher::compare_prompt;

let prompt = compare_prompt("Rc", "Arc", "English");
assert!(prompt.contains("Rc") && prompt.contains("Arc"));
```
