---
id: ide-app-extension-views-ideapp-install-extension
kind: doc
target_file: src/ide_app/extension_views.rs
line_hint: 59
author: 
created: 
title: "IdeApp::install_extension"
tags: []
signature: "pub (super) fn install_extension (& mut self , cx : & mut Cx , folder : & std :: path :: Path ,) -> Result < String , String >"
---

Installs an extension from a folder into the project.

Installs the extension in `folder` into the project, live: its
actions, cards and views are there at once.
