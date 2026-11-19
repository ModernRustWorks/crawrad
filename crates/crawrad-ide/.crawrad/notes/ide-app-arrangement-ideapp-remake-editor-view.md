---
id: ide-app-arrangement-ideapp-remake-editor-view
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 315
author: 
created: 
title: "IdeApp::remake_editor_view"
tags: []
signature: "pub (super) fn remake_editor_view (& mut self , cx : & mut Cx)"
---

Re-populates the code editor view after state reload.

`load_state` remakes the code editor EMPTY; `colher_do_editor` took that
as an edit and Reset layout + Save wiped the open file (27/10/2026).
