---
id: ide-app-extension-views-ideapp-extension-views-actions
kind: doc
target_file: src/ide_app/extension_views.rs
line_hint: 153
author: 
created: 
title: "IdeApp::extension_views_actions"
tags: []
signature: "pub (super) fn extension_views_actions (& mut self , cx : & mut Cx , actions : & Actions)"
---

Handles clicks and filter changes in extension views.

A click on a row, and the filter typed: a header or a branch opens or
closes; an item is told to its extension and, when it points
somewhere, opened there.
