---
id: ide-app-dialogs-actions-ideapp-atender-actions
kind: doc
target_file: src/ide_app/dialogs/actions.rs
line_hint: 121
author: 
created: 
title: "IdeApp::atender_actions"
tags: []
signature: "pub (in super :: super) fn atender_actions (& mut self , cx : & mut Cx , actions : & Actions)"
---

Handles gestures for the ActionList Editor.

**No action rule lives here**: `New Action`, `Standard…`, and
`Delete` are the catalog actions `action.new`, `action.standard`, and
`edit.delete`, and moving is the document's `Command::Reorder` — the same
that the egui shell fires.

## Inside `atender_actions`: `let lst = dlg.portal_list(id!(act_list));`

A CLICK on an action selects it in the DOCUMENT — this is what makes the Object
Inspector behind the window show its properties, as in
a classic RAD IDE.
