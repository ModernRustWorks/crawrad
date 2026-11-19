---
id: chat-layout-switch-label
kind: doc
target_file: src/chat_layout.rs
line_hint: 247
author: 
created: 
title: "switch_label"
tags: []
signature: "pub fn switch_label (name : & str , on : bool) -> String"
---

A context switch's caption: `Form: ON`, `Sel: OFF`.

# Examples

```rust
use crawrad_agent::chat_layout::switch_label;

assert_eq!(switch_label("Form", true), "Form: ON");
```
