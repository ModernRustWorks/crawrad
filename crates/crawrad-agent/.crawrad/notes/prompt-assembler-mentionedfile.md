---
id: prompt-assembler-mentionedfile
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 143
author: 
created: 
title: "MentionedFile"
tags: []
signature: "pub struct MentionedFile"
---

An attached or mentioned file resolved from workspace or live memory buffers.

# Examples

```rust
use crawrad_agent::MentionedFile;

let mf = MentionedFile {
    path: "src/main.rs".to_string(),
    content: "fn main() {}".to_string(),
    is_truncated: false,
    total_lines: 1,
};
assert_eq!(mf.total_lines, 1);
```
