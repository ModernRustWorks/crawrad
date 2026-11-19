---
id: prompt-assembler-truncate-text-on-line-boundary
kind: doc
target_file: src/prompt_assembler.rs
line_hint: 188
author: 
created: 
title: "truncate_text_on_line_boundary"
tags: []
signature: "pub fn truncate_text_on_line_boundary (text : & str , max_bytes : usize) -> (String , bool , usize)"
---

Truncates text cleanly on line boundaries when exceeding byte limits.

Truncates text on a clean line boundary if it exceeds max_bytes, returning (truncated_text, was_truncated, total_lines).

# Examples

```rust
use crawrad_agent::truncate_text_on_line_boundary;

let (text, trunc, lines) = truncate_text_on_line_boundary("line1\nline2\n", 100);
assert!(!trunc);
assert_eq!(lines, 2);
```
