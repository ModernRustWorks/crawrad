---
id: ide-app-rename-ideapp-show-text-with-undo
kind: doc
target_file: src/ide_app/rename.rs
line_hint: 221
author: 
created: 
title: "IdeApp::show_text_with_undo"
tags: []
signature: "pub (super) fn show_text_with_undo (& mut self , cx : & mut Cx , file : Option < & Path > , text : & str)"
---

Displays buffer text for a file, preserving undo history for background edits.

A file a rename changed while hidden is shown: its text from before,
then the rename on top as one undo step. Else just the text.
