---
id: ide-app-extension-views-ideapp-write-view-row
kind: doc
target_file: src/ide_app/extension_views.rs
line_hint: 104
author: 
created: 
title: "IdeApp::write_view_row"
tags: []
signature: "pub (super) fn write_view_row (cx : & mut Cx , item : & WidgetRef , row : & ViewRow)"
---

Draws an individual extension view row widget.

One row: a view's header (▾/▸, title, count) or an item (▾/▸ when it
has children, the dot of its kind, label, detail), indented by depth.
