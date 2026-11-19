---
id: ide-app-rename-ideapp-apply-rename
kind: doc
target_file: src/ide_app/rename.rs
line_hint: 164
author: 
created: 
title: "IdeApp::apply_rename"
tags: []
signature: "fn apply_rename (& mut self , cx : & mut Cx)"
---

Applies the planned rename edits across affected files in memory.

Apply: every file, in memory. The one in front as ONE undo step; the
others become unsaved edits whose undo is replayed when they are shown.
