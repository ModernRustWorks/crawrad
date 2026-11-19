---
id: chat-layout-permission-choices
kind: doc
target_file: src/chat_layout.rs
line_hint: 217
author: 
created: 
title: "PERMISSION_CHOICES"
tags: []
signature: "pub const PERMISSION_CHOICES : [& str ; 4] = [\"allow_once\" , \"allow_always\" , \"reject_once\" , \"reject_always\"]"
---

Permission card answer choices.

The permission card's four answers, as their buttons end (`btn_perm_allow_once`…)
and as `chat.permission {choice}` names them.

# Examples

```rust
use crawrad_agent::chat_layout::PERMISSION_CHOICES;

assert_eq!(PERMISSION_CHOICES.len(), 4);
```
