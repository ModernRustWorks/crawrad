---
id: ide-app-chat-ideapp-warm-live-agent
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 557
author: 
created: 
title: "IdeApp::warm_live_agent"
tags: []
signature: "pub fn warm_live_agent (& mut self , cx : & mut Cx)"
---

Warms the live ACP client.

Warms the live ACP client — never in unit tests, which read the real `%APPDATA%`.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.busy);
```
