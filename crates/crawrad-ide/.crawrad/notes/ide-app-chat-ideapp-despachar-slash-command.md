---
id: ide-app-chat-ideapp-despachar-slash-command
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 19
author: 
created: 
title: "IdeApp::despachar_slash_command"
tags: []
signature: "pub (super) fn despachar_slash_command (& mut self , cx : & mut Cx , prompt : & str) -> Slash"
---

The AI panel MOCKUP's response: it says it is not connected.

# Why there is no response other than this one

Until this iteration the panel chose among three responses depending on the word
present in the prompt, and all three **claimed to have done what they had not**:

- `"All constraints and anchors are correctly aligned to the 8px grid"`
  was a VERDICT on the user's form, and not a single line of this
  shell measured constraint, anchor, or grid;
- `"I've generated the native event handler"` generated nothing — not a single
  byte went into any file;
- `"Would you like me to insert this handler into the form code?"`
  offered a path that does not exist.

The panel STAYS — it is a deliberate mockup, and real chat belongs to
Phase 4. What leaves is the pretense. The rule is the same one this shell
applied to the status bar: **what is not wired says it is not wired**, and a
plausible number is worse than an em dash.
Dispatches slash commands (`/slash-commands`) invoked in the AI chat.
