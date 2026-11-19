---
id: ide-app-lsp-service-ideapp-format-document
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 560
author: 
created: 
title: "IdeApp::format_document"
tags: []
signature: "fn format_document (& mut self)"
---

Requests rust-analyzer to format the currently active editable document.

Format Document: asks; [`Self::poll_format`] applies. Refused on a
file that takes no typing (std, crates, the generated form source).
