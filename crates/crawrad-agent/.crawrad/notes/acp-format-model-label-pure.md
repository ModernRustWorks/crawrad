---
id: acp-format-model-label-pure
kind: doc
target_file: src/acp.rs
line_hint: 906
author: 
created: 
title: "format_model_label_pure"
tags: []
signature: "pub fn format_model_label_pure (model : & str) -> String"
---

Formats the AI model label safely for UI display.

Formats the AI model label purely and panic-safely for multibyte UTF-8.

# Examples

```rust
use crawrad_agent::format_model_label_pure;

assert_eq!(format_model_label_pure("sonnet"), "Sonnet 3.7 ▾");
```
