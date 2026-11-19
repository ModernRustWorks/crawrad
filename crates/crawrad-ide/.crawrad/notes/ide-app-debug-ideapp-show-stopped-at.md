---
id: ide-app-debug-ideapp-show-stopped-at
kind: doc
target_file: src/ide_app/debug.rs
line_hint: 99
author: 
created: 
title: "IdeApp::show_stopped_at"
tags: []
signature: "fn show_stopped_at (& mut self , cx : & mut Cx , file : PathBuf , line : u32 , reason : & str)"
---

Navigates the editor to the stopped debugger location.

The program stopped: its file in the editor, in the Code view, with the
caret on the line — the arrow is drawn by `sync_breakpoints`.
