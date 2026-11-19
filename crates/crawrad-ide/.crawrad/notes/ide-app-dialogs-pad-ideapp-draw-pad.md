---
id: ide-app-dialogs-pad-ideapp-draw-pad
kind: doc
target_file: src/ide_app/dialogs/pad.rs
line_hint: 14
author: 
created: 
title: "IdeApp::draw_pad"
tags: []
signature: "pub (in super :: super) fn draw_pad (& mut self , cx : & mut Cx)"
---

Draws the `Align` SELECTOR on screen, and hides it when there is none.

Derived from the field each frame, rather than its own visibility state:
two states answering the same question are two states that diverge,
which is the same argument as `painel_a_vista`.

**It ended up alone here on 2026-09-17**: the text pad that used to share
this function became the Lines Editor, with a dialog frame and catalog title
([`IdeApp::draw_lines`]). What remained is the only Inspector window
that is not a real dialog — it is a drawing of the form where one clicks
a region, and it is born ANCHORED to the panel, not centered.

#### At `if self.pad_na_tela.as_deref() == Some(&pad.key) {`

THE LABEL is only written when the window OPENS, and never on every frame:
writing to a label marks it as dirty, resulting in a window that never
sleeps. It is the same reason as for the lines scratchpad ([`Edicao`]) and
[`IdeApp::barra_na_tela`].

## Inside `draw_pad`: `if let Some(nome) = crawrad_registry::ALIGN_OPTIONS.first() {`

The label for the `None` button comes from the CATALOG, not from this shell: it is the
first one in `ALIGN_OPTIONS`, and a name written here would be a second
list of alignments diverging from the first.
