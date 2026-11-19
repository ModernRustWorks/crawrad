---
id: ide-app-lsp-service-ideapp-lsp-apply-occurrences
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 202
author: 
created: 
title: "IdeApp::lsp_apply_occurrences"
tags: []
signature: "fn lsp_apply_occurrences (& mut self , cx : & mut Cx , lsp : & Arc < crawrad_lsp :: LspClient > , file : & Path)"
---

Applies occurrences of the identifier under the caret.

The occurrences of the name under the caret, lit behind the text — the
caret's place is asked about once, when it rests there.
