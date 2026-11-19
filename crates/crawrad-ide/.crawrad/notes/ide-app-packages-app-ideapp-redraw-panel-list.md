---
id: ide-app-packages-app-ideapp-redraw-panel-list
kind: doc
target_file: src/ide_app/packages_app.rs
line_hint: 210
author: 
created: 
title: "IdeApp::redraw_panel_list"
tags: []
signature: "pub (super) fn redraw_panel_list (& mut self , cx : & mut Cx , p : Panel , caminho : & [LiveId])"
---

Requests a redraw of a dock panel list.

**It does not come for free with `self.ui.redraw(cx)`**, and that was measured on
screen: `Dock` draws its children on its own, and a `PortalList`
living inside an item of it stays with the previous frame's screen
while the state underneath has already changed. With nothing in the log — which is the worst
way for a panel to lie.
