---
id: ide-app-arrangement-ideapp-mover-painel
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 232
author: 
created: 
title: "IdeApp::mover_painel"
tags: []
signature: "pub (super) fn mover_painel (& mut self , cx : & mut Cx , p : Panel , lado : crawrad_actions :: DockSide)"
---

Docks the panel against an edge of the window.

**The arithmetic is the framework's, and on purpose.** `drop_move` receives a
POINT and decides on its own what to do with it — the same function that drag
`Drop` calls (`dock.rs:1258` → `handle_drop`). Choosing here
a point 2 px away from the dock edge is saying "drop here", and the result is
indistinguishable from dragging there with the mouse.

This is what makes the command test cover dragging: there is no second
code path to diverge.
