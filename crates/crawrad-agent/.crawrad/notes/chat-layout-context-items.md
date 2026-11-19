---
id: chat-layout-context-items
kind: doc
target_file: src/chat_layout.rs
line_hint: 204
author: 
created: 
title: "CONTEXT_ITEMS"
tags: []
signature: "pub const CONTEXT_ITEMS : [& str ; 6] = [\"media\" , \"mentions\" , \"actions\" , \"memory\" , \"mcp\" , \"agent\"]"
---

Context menu item identifiers.

The `+` menu's items, as their widgets end (`btn_ai_media`…) and as
`chat.context_menu {item}` names them (#420).

# Examples

```rust
use crawrad_agent::chat_layout::CONTEXT_ITEMS;

assert!(CONTEXT_ITEMS.contains(&"media"));
```
