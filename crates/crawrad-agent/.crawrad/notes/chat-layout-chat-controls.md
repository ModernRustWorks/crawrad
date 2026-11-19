---
id: chat-layout-chat-controls
kind: doc
target_file: src/chat_layout.rs
line_hint: 176
author: 
created: 
title: "CHAT_CONTROLS"
tags: []
signature: "pub const CHAT_CONTROLS : [& str ; 20] = [\"btn_ai_new\" , \"btn_ai_clear\" , \"btn_ai_history\" , \"btn_ai_settings\" , \"btn_form_toggle\" , \"btn_sel_toggle\" , \"btn_ai_attach\" , \"btn_prompt_send\" , \"btn_prompt_stop\" , \"btn_ai_mic\" , \"btn_ai_media\" , \"btn_ai_mentions\" , \"btn_ai_actions\" , \"btn_ai_memory\" , \"btn_ai_mcp\" , \"btn_ai_agent\" , \"btn_perm_allow_once\" , \"btn_perm_allow_always\" , \"btn_perm_reject_once\" , \"btn_perm_reject_always\" ,]"
---

The list of chat control widget names reported by view inspection.

The chat's controls `view.get_chat` reports (rect + visible), by widget name,
so a script or a video can click them for real (#420).

# Examples

```rust
use crawrad_agent::chat_layout::CHAT_CONTROLS;

assert!(CHAT_CONTROLS.contains(&"btn_ai_new"));
```
