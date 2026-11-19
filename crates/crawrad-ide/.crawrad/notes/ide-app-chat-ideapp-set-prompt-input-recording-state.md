---
id: ide-app-chat-ideapp-set-prompt-input-recording-state
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 1069
author: 
created: 
title: "IdeApp::set_prompt_input_recording_state"
tags: []
signature: "pub (super) fn set_prompt_input_recording_state (& mut self , cx : & mut Cx , recording : bool , texto : Option < & str > ,)"
---

Updates the prompt state during and after audio recording.

Updates the visual and editing state of the AI prompt and send button
during and after audio recording.

When `recording` is true:
- Prompt field becomes read-only / disabled
- Text color is dimmed (#6b6b6e)
- Text cursor is hidden (#0000)
- Send button is disabled (gray and locked)

When `recording` is false:
- Prompt field is re-enabled (editing allowed)
- Normal colors (#e7e7ea / #ffffff) and active cursor (#007acc)
- Send button is re-enabled (blue with white arrow, clickable)
