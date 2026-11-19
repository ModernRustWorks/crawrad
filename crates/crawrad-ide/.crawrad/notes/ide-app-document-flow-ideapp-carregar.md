---
id: ide-app-document-flow-ideapp-carregar
kind: doc
target_file: src/ide_app/document_flow.rs
line_hint: 11
author: 
created: 
title: "IdeApp::carregar"
tags: []
signature: "pub (super) fn carregar (& mut self , cx : & mut Cx)"
---

Opens the form from disk and delivers it to the canvas.

**No `include_str!`.** Until this iteration the shell embedded a
`.crawform` at compile time, and therefore opening another file was
impossible by construction. The search order lives in
[`documento::candidatos`] and failure is reported in full — no
fake form is created here.
