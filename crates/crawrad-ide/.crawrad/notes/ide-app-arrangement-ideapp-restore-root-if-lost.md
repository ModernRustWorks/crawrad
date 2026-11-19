---
id: ide-app-arrangement-ideapp-restore-root-if-lost
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 107
author: 
created: 
title: "IdeApp::restore_root_if_lost"
tags: []
signature: "pub (super) fn restore_root_if_lost (& mut self , cx : & mut Cx) -> bool"
---

Restores `root` when the last command emptied a group under it.

Puts `root` back when the last command emptied a group under it (#272)
— see [`layout::with_root_restored`]. Returns whether it had to.
