---
id: ide-app-dialogs-pad-ideapp-atender-pad
kind: doc
target_file: src/ide_app/dialogs/pad.rs
line_hint: 63
author: 
created: 
title: "IdeApp::atender_pad"
tags: []
signature: "pub (in super :: super) fn atender_pad (& mut self , cx : & mut Cx , actions : & Actions)"
---

Handles gestures for the `Align` selector.

**No `Transaction` is born here either**: what the region returns is
the name declared by the catalog, and it is dispatched via `component.set_property`.

## Inside `atender_pad`: `let mut escolhido: Option<&'static str> = None;`

Five regions and the `None` button. The name of each comes from
[`ALIGN_ZONES`], which is checked against the catalog by test —
no alignment is invented in this shell.
