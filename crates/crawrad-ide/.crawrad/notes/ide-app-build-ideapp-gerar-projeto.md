---
id: ide-app-build-ideapp-gerar-projeto
kind: doc
target_file: src/ide_app/build.rs
line_hint: 329
author: 
created: 
title: "IdeApp::gerar_projeto"
tags: []
signature: "pub (super) fn gerar_projeto (& mut self , cx : & mut Cx)"
---

Writes to disk what the next `Run` will compile — the `Generate`.

**No generation rule lives here.** The plan, the pre-pass pruning of
stale bindings, and writing are `crate::generator`, which is the same
framework-free module that the egui shell calls — it was because it was
extracted from inside egui's `Ide` that this method could exist.

`&mut ()` is the `crate::generator::PendingEdits` of a shell **without
an open text editor**: this shell displays code (`F12`), but does not
edit it. The day it does edit, this is what becomes the editor's type, and the order
"save what is being typed BEFORE the pre-pass reads the disk"
(issue #49) is already guaranteed by `prepare`.

#### At `if let Some(aviso) = self`

**THIRD-PARTY COMPONENT DEPENDENCIES.**

Without this, the code comes out right and the project DOES NOT COMPILE:
`gauge_crawrad::Gauge` only resolves if `Cargo.toml` depends on the crate.

After writing, not before: if generation fails, there is no reason to
touch anyone's manifest.

## Inside `gerar_projeto`: `if let Some(tx) = plano.unlink()`

THE ONLY MUTATION GATE REMAINS THE DOCUMENT. The pre-pass
PROPOSES unlinking stale bindings; what writes is
`apply`, with `Ctrl+Z` on the other side. ONE transaction for all.

## Inside `gerar_projeto`: `self.base_na_vista = texto.clone();`

The baseline too: the generation just wrote this text, so the tab
must not look dirty from work nobody did.

## Inside `gerar_projeto`: `if self.host.center().is_text() {`

The editor rereads the disk the generation just rewrote: otherwise a
pruned handler stays on screen after Ctrl+S (owner, 17/09/2026).

## Inside `gerar_projeto`: `self.redraw_panel_list(cx, Panel::Explorer, id!(explorer_body.explo...`

The Explorer changes with generation: what it showed as planned now
exists. `self.ui.redraw` does not reach a docked `PortalList`.

## Inside `gerar_projeto`: `if !had_target && crawrad_project::generator::has_a_target(&destino) {`

rust-analyzer read the project before it had a `main.rs`: it must
read it again, or it keeps answering nothing.

## Inside `gerar_projeto`: `Err(e) => {`

Editor saving was refused. We do not generate on top: the
pre-pass would evaluate a disk state that the user does not control.
