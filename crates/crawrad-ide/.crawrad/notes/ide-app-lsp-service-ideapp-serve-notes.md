---
id: ide-app-lsp-service-ideapp-serve-notes
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 219
author: 
created: 
title: "IdeApp::serve_notes"
tags: []
signature: "fn serve_notes (& mut self , cx : & mut Cx)"
---

Serves note icons in the editor margin and handles click interactions.

The note icons in the margin: a click opens the note, and the anchors
drawn without a preview get one, read from disk once each.
