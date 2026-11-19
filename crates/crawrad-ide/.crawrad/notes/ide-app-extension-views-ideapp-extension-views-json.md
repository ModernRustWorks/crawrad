---
id: ide-app-extension-views-ideapp-extension-views-json
kind: doc
target_file: src/ide_app/extension_views.rs
line_hint: 206
author: 
created: 
title: "IdeApp::extension_views_json"
tags: []
signature: "pub (super) fn extension_views_json (& self , cx : & Cx) -> serde_json :: Value"
---

Returns JSON state for extension views, filter and row rects.

What `view.get_extension_views` reads: the views, the filter, and
each drawn row with its rect in window pixels.
