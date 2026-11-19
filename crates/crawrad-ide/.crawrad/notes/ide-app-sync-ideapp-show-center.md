---
id: ide-app-sync-ideapp-show-center
kind: doc
target_file: src/ide_app/sync.rs
line_hint: 260
author: 
created: 
title: "IdeApp::show_center"
tags: []
signature: "pub (super) fn show_center (& mut self , cx : & mut Cx , nova : source :: Center)"
---

Shows the centre view asked for and hides the others.

Design, code, history and welcome share the centre column: exactly one of
them is visible, and the host records which.

**What was typed beats the disk.** Going back to a tab has to give back what
was written in it, not the file as it was before — see [`IdeApp::edicoes`].

The editor footer needs the line the cursor is on to turn a byte into a
column, and how the file was written. Both are kept here, the only place the
view's text passes through — see [`editor_footer`].
