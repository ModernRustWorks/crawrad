---
id: ide-app-key-dispatch-ideapp-seta
kind: doc
target_file: src/ide_app/key_dispatch.rs
line_hint: 130
author: 
created: 
title: "IdeApp::seta"
tags: []
signature: "pub (super) fn seta (& mut self , dx : f32 , dy : f32 , modo : crawrad_designer :: ArrowNudge)"
---

The arrow keys: move or resize the selection.

**It is not a catalogue action, and could not be**: it is a designer gesture, like
dragging. The transaction comes ready from `crawrad-designer`, which already knows
grid, snap, and the refusal of what is locked — this shell recalculates
none of that (ADR 0027), and that is why it receives the MODE and asks for the
step instead of bringing a number.
