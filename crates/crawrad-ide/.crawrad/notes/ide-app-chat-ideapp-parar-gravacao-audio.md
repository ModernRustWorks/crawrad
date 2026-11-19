---
id: ide-app-chat-ideapp-parar-gravacao-audio
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 1144
author: 
created: 
title: "IdeApp::parar_gravacao_audio"
tags: []
signature: "pub fn parar_gravacao_audio (& mut self , cx : & mut Cx)"
---

Stops recording and starts Whisper transcription in the background.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.audio_gravando);
```
