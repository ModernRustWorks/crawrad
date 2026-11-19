---
id: ide-app-chat-ideapp-bombear-ia
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 935
author: 
created: 
title: "IdeApp::bombear_ia"
tags: []
signature: "pub fn bombear_ia (& mut self , cx : & mut Cx)"
---

Processes periodic AI pulses in the interface loop.

Processes periodic AI pulses (chat responses, connection tests, and probes).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.busy);
```
