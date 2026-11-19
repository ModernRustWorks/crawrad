---
id: ide-app-debug-ideapp-add-watch
kind: doc
target_file: src/ide_app/debug.rs
line_hint: 206
author: 
created: 
title: "IdeApp::add_watch"
tags: []
signature: "pub (super) fn add_watch (& mut self , cx : & mut Cx , expression : Option < String >)"
---

Adds a watch expression to the debugging session.

`debug.add_watch` (`Ctrl+F5` in the classic key scheme): the expression given, or the
editor's selection, or the word under the caret — a classic Add Watch
dialog starts filled the same way.
