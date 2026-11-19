---
id: ide-app-chat-ideapp-iniciar-gravacao-audio
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 1106
author: 
created: 
title: "IdeApp::iniciar_gravacao_audio"
tags: []
signature: "pub fn iniciar_gravacao_audio (& mut self , cx : & mut Cx)"
---

Starts capturing audio via the user's microphone (CPAL).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.audio_gravando);
```
