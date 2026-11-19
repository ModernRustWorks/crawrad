---
id: ide-app-dialogs-lines-ideapp-draw-lines
kind: doc
target_file: src/ide_app/dialogs/lines.rs
line_hint: 34
author: 
created: 
title: "IdeApp::draw_lines"
tags: []
signature: "pub (in super :: super) fn draw_lines (& mut self , cx : & mut Cx)"
---

Draws the Lines Editor on screen, and hides it when none is open.

**No label comes from the DSL**: the title is
[`lines_editor::editor_title`] — which yields `Memo Lines Editor` today and
`Query SQL Editor` the day someone declares `multilinha` on a
`Query`, without a single line changing here —, and the count is
`lines_editor::resumo`, the SAME one that the collapsed row of the Inspector
shows.

## Inside `draw_lines`: `if self.lines_na_tela {`

THE FIELD is only written when the window OPENS — see
[`IdeApp::lines_na_tela`].

## Inside `draw_lines`: `let dono = format!("{raiz}.{}.{}", estado.node_name, estado.prop_di...`

`form_main.memo1.Lines` — the egui header, with the property
included: this window is not for a single component.
