---
id: ide-app-chat-bar-ideapp-position-config-menu
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 232
author: 
created: 
title: "IdeApp::position_config_menu"
tags: []
signature: "pub fn position_config_menu (& mut self , cx : & mut Cx , category : & str)"
---

Positions the drop-down for the model or thought-level choice.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.chat.open_menu.is_none());
```
