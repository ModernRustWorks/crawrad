---
id: ide-app-dialogs-imagelist-ideapp-acrescentar-a-galeria
kind: doc
target_file: src/ide_app/dialogs/imagelist.rs
line_hint: 254
author: 
created: 
title: "IdeApp::acrescentar_a_galeria"
tags: []
signature: "pub (in super :: super) fn acrescentar_a_galeria (& mut self , cx : & mut Cx)"
---

Adds to the gallery the icon described by the three fields.

# What it rejects, and why the rejection belongs to the MODEL

Empty name, empty symbol, and duplicate name — the three rules are in
[`image_list_editor::icon_refusal`], which is the file that both
shells read. A validation written here would be a second idea of what
a valid icon is, and the one that would diverge.

# And it SAVES IMMEDIATELY

Without `OK` and without `Apply`: what the user expects from an *"add"* is that it sticks.
Without a place to save (a machine without `APPDATA`), the icon enters
this session's gallery and the bar indicates that it was not saved — promising what
is not kept is worse than rejecting.

## Inside `acrescentar_a_galeria`: `for (caminho, _) in NOVO_CAMPOS {`

THE FIELDS CLEAR, which is what indicates that the gesture finished — and leaves the
next icon one click away.
