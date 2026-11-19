---
id: ide-app-rename-ideapp-poll-rename
kind: doc
target_file: src/ide_app/rename.rs
line_hint: 72
author: 
created: 
title: "IdeApp::poll_rename"
tags: []
signature: "pub (super) fn poll_rename (& mut self , cx : & mut Cx)"
---

Polls LSP rename responses to open the dialog or display edits preview.

The server's answers: the name can be renamed (the dialog opens on
it), or the edits (checked, then previewed).
