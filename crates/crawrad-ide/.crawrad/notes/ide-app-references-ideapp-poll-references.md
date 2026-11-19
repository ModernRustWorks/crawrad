---
id: ide-app-references-ideapp-poll-references
kind: doc
target_file: src/ide_app/references.rs
line_hint: 44
author: 
created: 
title: "IdeApp::poll_references"
tags: []
signature: "pub (super) fn poll_references (& mut self , cx : & mut Cx)"
---

Polls LSP reference results and updates the references tool panel.

The answer: the places read against the text the editor holds, as
the panel's groups. Dropped when the file it was asked in changed.
