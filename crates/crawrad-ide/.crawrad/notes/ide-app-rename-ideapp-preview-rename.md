---
id: ide-app-rename-ideapp-preview-rename
kind: doc
target_file: src/ide_app/rename.rs
line_hint: 134
author: 
created: 
title: "IdeApp::preview_rename"
tags: []
signature: "fn preview_rename (& mut self , edits : & crawrad_lsp :: WorkspaceEdit)"
---

Validates workspace rename edits and prepares the preview stage.

The edits checked against the form (every file or none), then planned
against the text the editor holds.
