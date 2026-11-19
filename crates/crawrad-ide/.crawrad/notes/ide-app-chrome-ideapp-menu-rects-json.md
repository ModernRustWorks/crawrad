---
id: ide-app-chrome-ideapp-menu-rects-json
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 291
author: 
created: 
title: "IdeApp::menu_rects_json"
tags: []
signature: "pub (super) fn menu_rects_json (& self , cx : & Cx) -> serde_json :: Value"
---

Returns screen rects of menu bar and open popups for MCP.

Where the bar's buttons and the two menu panels are, for
`view.get_open_menu`: a real click needs a place on screen (#558).
