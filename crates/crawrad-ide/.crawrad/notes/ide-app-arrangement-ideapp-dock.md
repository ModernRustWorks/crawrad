---
id: ide-app-arrangement-ideapp-dock
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 27
author: 
created: 
title: "IdeApp::dock"
tags: []
signature: "pub (super) fn dock (& self) -> DockRef"
---

The shell's dock — the widget, not the state.

What stores the layout is `Dock` itself (`dock.rs:167` —
`dock_items: HashMap<LiveId, DockItem>`), which is why this shell
**does not have** a field indicating which panel is open. Such a field
would be a second copy of an already existing state, and the one that would diverge
the day the user dragged a tab — because dragging does not pass
through a single line of our code.
