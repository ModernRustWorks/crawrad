---
id: ide-app-lsp-service-ideapp-spin-lsp-icon
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 694
author: 
created: 
title: "IdeApp::spin_lsp_icon"
tags: []
signature: "pub (super) fn spin_lsp_icon (& mut self , cx : & mut Cx , event : & Event)"
---

Animates the LSP status bar icon while a task is in progress.

While the pill is busy, its arc turns a little every frame; idle, no
frame is asked for and the window sleeps.
