---
id: ide-app-extension-views-ideapp-follow-extension-views
kind: doc
target_file: src/ide_app/extension_views.rs
line_hint: 16
author: 
created: 
title: "IdeApp::follow_extension_views"
tags: []
signature: "pub (crate) fn follow_extension_views (& mut self , cx : & mut Cx)"
---

Updates extension views when the view store changes.

Every wake: the views read again when the store changed, the list
drawn again; the filter box followed.
