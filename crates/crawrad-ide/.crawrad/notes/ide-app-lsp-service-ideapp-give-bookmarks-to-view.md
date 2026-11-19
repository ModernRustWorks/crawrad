---
id: ide-app-lsp-service-ideapp-give-bookmarks-to-view
kind: doc
target_file: src/ide_app/lsp_service.rs
line_hint: 246
author: 
created: 
title: "IdeApp::give_bookmarks_to_view"
tags: []
signature: "fn give_bookmarks_to_view (& mut self , cx : & mut Cx)"
---

Transfers active file bookmarks to the code view widget.

A view holding no file's bookmarks (its text just changed) is given the
session's for the file in it — at the end of every event, because the
window's sync does not run after every way a file opens.
