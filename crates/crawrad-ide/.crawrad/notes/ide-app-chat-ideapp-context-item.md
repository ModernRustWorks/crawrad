---
id: ide-app-chat-ideapp-context-item
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 608
author: 
created: 
title: "IdeApp::context_item"
tags: []
signature: "pub fn context_item (& mut self , cx : & mut Cx , item : & str)"
---

One item of the `+` menu (#420): the click and `chat.context_menu {item}`.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.busy);
```
