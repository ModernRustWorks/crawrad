---
id: chat-layout-context-switches
kind: doc
target_file: src/chat_layout.rs
line_hint: 200
author: 
created: 
title: "CONTEXT_SWITCHES"
tags: []
signature: "pub const CONTEXT_SWITCHES : [(& str , & str) ; 2] = [(\"btn_form_toggle\" , \"Form\") , (\"btn_sel_toggle\" , \"Sel\")]"
---

The two context switches: their widget and their caption's name.

# Examples

```rust
use crawrad_agent::chat_layout::CONTEXT_SWITCHES;

assert_eq!(CONTEXT_SWITCHES.len(), 2);
```
