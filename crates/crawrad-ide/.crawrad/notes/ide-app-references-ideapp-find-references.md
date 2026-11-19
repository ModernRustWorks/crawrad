---
id: ide-app-references-ideapp-find-references
kind: doc
target_file: src/ide_app/references.rs
line_hint: 19
author: 
created: 
title: "IdeApp::find_references"
tags: []
signature: "pub (super) fn find_references (& mut self , cx : & mut Cx)"
---

Dispatches a request to find all references to the symbol at the caret.

`editor.find_references`: asks; [`Self::poll_references`] fills the
panel, which comes forward now and says it is looking.
