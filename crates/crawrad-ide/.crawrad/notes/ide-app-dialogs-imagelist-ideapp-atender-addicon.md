---
id: ide-app-dialogs-imagelist-ideapp-atender-addicon
kind: doc
target_file: src/ide_app/dialogs/imagelist.rs
line_hint: 208
author: 
created: 
title: "IdeApp::atender_addicon"
tags: []
signature: "pub (in super :: super) fn atender_addicon (& mut self , cx : & mut Cx , actions : & Actions)"
---

Handles the **Add Icons from Catalog** window.

## Inside `atender_addicon`: `if janela.button(id!(novo_add)).clicked(actions) {`

ADD TO GALLERY — and it saves to disk immediately, because what the
user expects from an "add" is for it to STAY.
