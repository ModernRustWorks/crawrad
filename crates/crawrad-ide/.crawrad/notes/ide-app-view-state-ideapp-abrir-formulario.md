---
id: ide-app-view-state-ideapp-abrir-formulario
kind: doc
target_file: src/ide_app/view_state.rs
line_hint: 332
author: 
created: 
title: "IdeApp::abrir_formulario"
tags: []
signature: "pub (crate) fn abrir_formulario (& mut self , cx : & mut Cx , caminho : & std :: path :: Path) -> bool"
---

Switches the open form to another one from the group.

**It settles what was being typed and REFUSES on top of unsaved
work**, which is the egui shell guard (`permissao_de_troca`) in
the form this shell has: the document revision against the saved
revision. Without this, a click on a tree row would silently
discard the form that the person just designed — and ADR 0016
exists precisely so this screen doesn't give the most expensive advice it
could give.

Returns **whether the document actually loaded**. The one needing the verdict is
[`IdeApp::reload_from_disk`], which goes through here on purpose —
reloading IS reopening — and cannot announce `reloaded` upon a refusal
or upon a file that failed to read.

#### At `self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explorer`

THE EXPLORER CHANGES WITH THE DOCUMENT: the open form in bold, the
active member, the generation plan, and the folds, which [`Folds`]
reconstructs on its own. Without this redraw the tree would continue
pointing to the previous form.
