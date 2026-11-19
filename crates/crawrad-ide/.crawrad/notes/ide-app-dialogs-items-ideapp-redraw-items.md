---
id: ide-app-dialogs-items-ideapp-redraw-items
kind: doc
target_file: src/ide_app/dialogs/items.rs
line_hint: 182
author: 
created: 
title: "IdeApp::redraw_items"
tags: []
signature: "pub (in super :: super) fn redraw_items (& mut self , cx : & mut Cx)"
---

Repaints the entire dialog after a gesture.

**The `sincronizar` first, and it is not pedantry**: the `Text:` field and the
button states are DERIVED from the selected item, and what writes them is
[`IdeApp::draw_items`], which only runs from there. Without this call, creating
a `New SubItem` left the field displaying the text of the PREVIOUS item —
measured on screen on 2026-09-16.

Then the list, because it is a `PortalList` and `self.ui.redraw`
alone does not reach it. It is the same pitfall as the Inspector and the Explorer.
