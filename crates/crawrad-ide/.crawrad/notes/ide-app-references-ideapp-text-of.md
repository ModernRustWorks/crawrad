---
id: ide-app-references-ideapp-text-of
kind: doc
target_file: src/ide_app/references.rs
line_hint: 126
author: 
created: 
title: "IdeApp::text_of"
tags: []
signature: "pub (super) fn text_of (& self , file : & Path) -> Option < String >"
---

Reads file content from the active view, an unsaved buffer, or disk.

A file's text as the editor holds it: the one in front, then an
unsaved buffer, then the disk.
