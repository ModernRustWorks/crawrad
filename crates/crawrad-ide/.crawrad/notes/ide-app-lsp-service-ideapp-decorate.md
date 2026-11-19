---
id: ide-app-lsp-service-ideapp-decorate
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 749
author: 
created: 
title: "IdeApp::decorate"
tags: []
signature: "pub (super) fn decorate (& mut self , cx : & mut Cx , diagnostics : & [crawrad_lsp :: Diagnostic] , enc : crawrad_lsp :: encoding :: PositionEncoding ,)"
---

Renders diagnostic squiggles and inline messages in the code view.

A file's diagnostics drawn in the view: one mark per range — the
squiggle, the text faded or struck through as its tags say (#361) —
and the messages after their lines.
