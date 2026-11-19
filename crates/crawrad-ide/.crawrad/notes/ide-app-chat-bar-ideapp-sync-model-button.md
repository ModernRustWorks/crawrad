---
id: ide-app-chat-bar-ideapp-sync-model-button
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 443
author: 
created: 
title: "IdeApp::sync_model_button"
tags: []
signature: "pub fn sync_model_button (& mut self , cx : & mut Cx)"
---

Writes the model and effort buttons from the live agent's menus (#402).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.busy);
```
