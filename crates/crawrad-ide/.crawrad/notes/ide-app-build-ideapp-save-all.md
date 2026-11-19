---
id: ide-app-build-ideapp-save-all
kind: doc
target_file: src/ide_app/build.rs
line_hint: 222
author: 
created: 
title: "IdeApp::save_all"
tags: []
signature: "pub (super) fn save_all (& mut self , cx : & mut Cx) -> bool"
---

Saves all open editor buffers and modified forms.

**Save All**: the code of every tab (each in its own encoding), then
the form if it changed — code first, because the generation reads the
user's unit from the disk. `true` when everything was written; a file
that failed stays dirty and the status bar names it — one in review
too: it waits, the others are written (#559: one review blocked all).
