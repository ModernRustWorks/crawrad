---
id: ide-app-build-ideapp-write-editor-buffers
kind: doc
target_file: src/ide_app/build.rs
line_hint: 245
author: 
created: 
title: "IdeApp::write_editor_buffers"
tags: []
signature: "pub (super) fn write_editor_buffers (& mut self , cx : & mut Cx) -> (usize , Vec < String >)"
---

Writes every editor buffer to its file in its encoding.

Every editor buffer to its file, each in its own encoding: how many
were written and what failed. Shared by Save All and auto save (#555).
