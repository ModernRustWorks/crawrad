---
id: ide-app-lsp-service-ideapp-goto-definition
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 361
author: 
created: 
title: "IdeApp::goto_definition"
tags: []
signature: "fn goto_definition (& mut self , cx : & mut Cx)"
---

Asks rust-analyzer where the name at the caret is defined.

Asks rust-analyzer where the name at the caret is defined; the
answer arrives in [`Self::lsp_poll_definition`].
