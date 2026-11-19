---
id: edit-refusal
kind: doc
target_file: src/edit.rs
line_hint: 8
author: 
created: 
title: "Refusal"
tags: []
signature: "pub type Refusal = (& 'static str , String)"
---

A refusal: the tool's (code, sentence).

# Examples

```rust
use crawrad_mcp::edit::Refusal;

let r: Refusal = ("invalid-args", "Missing argument".to_string());
assert_eq!(r.0, "invalid-args");
```
