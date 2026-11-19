---
id: ide-app-arrangement-ideapp-load-state
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 14
author: 
created: 
title: "IdeApp::load_state"
tags: []
signature: "pub (super) fn load_state (& mut self , cx : & mut Cx , arranjo : std :: collections :: HashMap < LiveId , DockItem > ,)"
---

Reloads the entire dock arrangement.

Via `borrow_mut` and not via `DockRef`: `load_state` is a method of
`Dock` and was **not** promoted to `DockRef` (`dock.rs:983` — neighbor
methods `clone_state` and `check_and_clear_need_save` were). It is a framework
limitation, and the codebase accepts it rather than working around it: three lines of
borrowing are worth less than a fork.

The panels are filled again right after: see [`Self::remake_editor_view`].
