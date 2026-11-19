---
id: ide-app-chat-ideapp-toggle-audio-mic
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 1032
author: 
created: 
title: "IdeApp::toggle_audio_mic"
tags: []
signature: "pub fn toggle_audio_mic (& mut self , cx : & mut Cx)"
---

Toggles between recording the microphone and transcribing it.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(!app.chat.audio_gravando);
```
