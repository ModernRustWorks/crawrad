---
id: ide-app-lsp-service-ideapp-draw-lsp-cell
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 669
author: 
created: 
title: "IdeApp::draw_lsp_cell"
tags: []
signature: "fn draw_lsp_cell (& mut self , cx : & mut Cx , cell : LspCell)"
---

Updates the status bar LSP pill icon and progress details.

The pill (its icon's phase and the server's name) and, beside it, the
progress line cut to its fixed cell, so the message never jumps (#383).
