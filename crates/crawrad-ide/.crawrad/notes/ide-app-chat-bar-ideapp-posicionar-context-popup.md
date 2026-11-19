---
id: ide-app-chat-bar-ideapp-posicionar-context-popup
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 272
author: 
created: 
title: "IdeApp::posicionar_context_popup"
tags: []
signature: "pub fn posicionar_context_popup (& mut self , cx : & mut Cx)"
---

Positions the context popup (+) on the prompt bar.

Positions the context popup (+) anchored directly above the prompt bar and the `+` button.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.busy);
```
