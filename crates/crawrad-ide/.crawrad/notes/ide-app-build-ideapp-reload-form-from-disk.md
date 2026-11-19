---
id: ide-app-build-ideapp-reload-form-from-disk
kind: doc
target_file: src/ide_app/build.rs
line_hint: 282
author: 
created: 
title: "IdeApp::reload_form_from_disk"
tags: []
signature: "pub (super) fn reload_form_from_disk (& mut self , cx : & mut Cx)"
---

Reloads the form from disk — the catalog's `Reload`.

Reloading IS reopening, and that is why it goes through
[`IdeApp::abrir_formulario`]: a second path to reopen is a
path that forgets to reset something, and this codebase has already paid for
that oversight twice.

## Inside `reload_form_from_disk`: `self.status = Some(format!("{} is gone from disk", caminho.display(...`

The document vanished from disk between opening and reloading. Saying that is
all that belongs here: replacing what is on screen with something else would be the
most direct way to make someone think they lost their work.
