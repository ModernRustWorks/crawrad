---
id: ide-app-dialogs-items-ideapp-draw-items
kind: doc
target_file: src/ide_app/dialogs/items.rs
line_hint: 33
author: 
created: 
title: "IdeApp::draw_items"
tags: []
signature: "pub (in super :: super) fn draw_items (& mut self , cx : & mut Cx)"
---

Draws the Items Editor on screen, and hides it when none is open.

Derived from the field each frame, like [`IdeApp::draw_pad`]: two
states answering the same question are two states that diverge.

**No label comes from the DSL.** The title is `editor_title` — the
same one that gives `ListBox Items Editor` and `ComboBox Items Editor` there —, and
the five commands come from [`ITEMS_BOTOES`]. A second list written
in the DSL would be the one to diverge.

#### At `let raiz = self.form_name();`

`form_main.tree_view1` — the FORM name and the component name, which
is the egui header. The root name comes from the document, as everywhere
in this shell: `id` is identity and `Name` is what is read
(ADR 0008).

## Inside `draw_items`: `let tem_sel = fields.has_selection();`

WHAT CANNOT BE DONE STAYS, AND STAYS DIMMED — the house rule, and that of
a classic RAD IDE: without a selected item there is nothing to delete or move, and
in a flat list `New SubItem` means nothing.

## Inside `draw_items`: `if self.items_na_tela == selecionado {`

THE FIELD is only rewritten when the ITEM changes — see
[`IdeApp::items_na_tela`].
