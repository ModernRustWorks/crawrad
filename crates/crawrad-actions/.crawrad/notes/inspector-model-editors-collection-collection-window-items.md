---
id: inspector-model-editors-collection-collection-window-items
kind: doc
target_file: src/inspector_model/editors/collection.rs
line_hint: 150
author: 
created: 
title: "Collection::window_items"
tags: []
signature: "fn window_items (& self) -> Option < bool >"
---

THE COLLECTION OPENS THE ITEMS EDITOR, not the text block.

The block was the workaround while the dialog did not exist on the
Makepad side, and the owner rejected it on 2026-09-16 looking at the screen:
*"its dialog is not this one, it is similar to that of a classic RAD IDE"*. The model is the `items_editor`
of the egui shell — tree on the left, `New Item`/`New SubItem`/`Delete`/
`Move Up`/`Move Down` in the middle, item properties on the right.

The `bool` is the one from the CATALOG, and travels from here instead of being asked
again there — see [`PropEditor::window_items`].
