---
id: ide-app-dialogs-items-ideapp-atender-items
kind: doc
target_file: src/ide_app/dialogs/items.rs
line_hint: 90
author: 
created: 
title: "IdeApp::atender_items"
tags: []
signature: "pub (in super :: super) fn atender_items (& mut self , cx : & mut Cx , actions : & Actions)"
---

Handles gestures for the Items Editor.

**No collection rules live here**: the five buttons call the five
functions in [`items_editor`], which are the same ones the egui shell calls.
What this function does is translate clicks into calls, and return the value
through the usual gate — the catalog's `component.set_property`.

#### At `let lista = dlg.portal_list(id!(items_list));`

A CLICK ON A ROW of the tree selects the item. The index comes from
`PortalList` itself, and the row list is reassembled from the same
function used for drawing — there is no widget table to keep up to
date.

## Inside `atender_items`: `self.items_na_tela = None;`

The tree changed, and with it the selected item: the text field
must be rewritten in the next frame.

## Inside `atender_items`: `self.items_na_tela = None;`

Switching items FORCES rewriting the text field — which is
what `items_na_tela` tracks.

## Inside `atender_items`: `let typed = |caminho: &[LiveId]| dlg.text_input(caminho).changed(ac...`

What is typed or ticked goes into the state, not the document: OK
and Apply take it there, as in egui.
