---
id: ide-app-chat-bar-ideapp-update-chat-buttons-state
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 405
author: 
created: 
title: "IdeApp::update_chat_buttons_state"
tags: []
signature: "pub fn update_chat_buttons_state (& mut self , cx : & mut Cx)"
---

Toggles visibility between the Send and Stop buttons.

Toggles visibility between the Send and Stop buttons according to the `ai_busy` state.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.busy);
```
