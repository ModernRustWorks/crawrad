---
id: ide-app-explorer-actions-ideapp-explorer-drawn
kind: doc
target_file: src/ide_app/explorer_actions.rs
line_hint: 96
author: 
created: 
title: "IdeApp::explorer_drawn"
tags: []
signature: "pub (super) fn explorer_drawn (rows : & [explorer :: Row] , rects : & [(usize , Rect)] ,) -> serde_json :: Value"
---

Returns JSON for the drawn Explorer rows and their rects.

What `view.get_explorer` reads (#412): the rows the last frame DREW, with
rects — never rebuilt on read, so an unredrawn panel shows what it shows.
