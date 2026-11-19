---
id: ide-app-dialogs-imagelist-ideapp-atender-imagelist
kind: doc
target_file: src/ide_app/dialogs/imagelist.rs
line_hint: 129
author: 
created: 
title: "IdeApp::atender_imagelist"
tags: []
signature: "pub (in super :: super) fn atender_imagelist (& mut self , cx : & mut Cx , actions : & Actions)"
---

Handles gestures for the ImageList Editor and the inner window.

## Inside `atender_imagelist`: `if self.imagelist.as_ref().is_some_and(|e| e.add_dialog_open) {`

THE INNER WINDOW FIRST, and with `return`: while it is
open, it is the modal one — handling both in the same frame would let a
click on its `Add` also hit the `Add…` behind it.
