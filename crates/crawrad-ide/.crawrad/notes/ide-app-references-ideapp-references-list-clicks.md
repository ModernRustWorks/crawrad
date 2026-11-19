---
id: ide-app-references-ideapp-references-list-clicks
kind: doc
target_file: src/ide_app/references.rs
line_hint: 189
author: 
created: 
title: "IdeApp::references_list_clicks"
tags: []
signature: "pub (super) fn references_list_clicks (& mut self , cx : & mut Cx , actions : & Actions)"
---

Handles user clicks on rows in the references list panel.

A click on a row: a file opens or closes, an occurrence opens its
file with the caret on the name (locked when it is not the user's).
