---
id: prompt-assembler-extract-selected-text
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 552
author: 
created: 
title: "extract_selected_text"
tags: []
signature: "pub fn extract_selected_text (texto : & str , start_line : usize , start_byte : usize , end_line : usize , end_byte : usize ,) -> String"
---

Extracts selected text from 1-based line and byte ranges.

Extracts selected text from a full string given 1-based start and end line/byte positions.

# Examples

```rust
use crawrad_agent::prompt_assembler::extract_selected_text;

let text = extract_selected_text("hello world", 1, 0, 1, 5);
assert_eq!(text, "hello");
```
