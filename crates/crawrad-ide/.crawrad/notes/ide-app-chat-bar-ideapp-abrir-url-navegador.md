---
id: ide-app-chat-bar-ideapp-abrir-url-navegador
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 453
author: 
created: 
title: "IdeApp::abrir_url_navegador"
tags: []
signature: "pub fn abrir_url_navegador (& mut self , url : & str)"
---

Opens a URL in the operating system's default browser.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
app.abrir_url_navegador("https://crawrad.org");
assert!(app.status.is_some());
```
