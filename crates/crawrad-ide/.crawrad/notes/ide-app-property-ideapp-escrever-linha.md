---
id: ide-app-property-ideapp-escrever-linha
kind: doc
target_file: src/ide_app/property.rs
line_hint: 126
author: 
created: 
title: "IdeApp::escrever_linha"
tags: []
signature: "pub (super) fn escrever_linha (cx : & mut Cx2d , item : & WidgetRef , novo : bool , indice : usize , linha : & inspector :: Row , rascunho : Option < & Edicao > , rascunho_evento : Option < & EventEdit > , enums : & mut CacheDeOpcoes , mark : Option < bool > ,)"
---

Writes an Inspector row into the widget that its template created.

It is an associated function, not a method, for a borrow reason: it touches
two fields of `self` — the draft and the combo cache — while the
`PortalList` is borrowed from `WidgetRef`. By taking both by
reference, the compiler sees that they are disjoint.

#### At `let mut clear_shown = false;`

THERE IS NO `match` BY EDITOR TYPE HERE, and that is what the decomposition
bought: the editor says what goes in which slot, and this loop writes it.
Adding a property type is a file in
`ide_app/inspector/editors/`, and not a single line here.
A picture's `×` shows only with a picture, and a reused `RowOpen` must
not keep it on a `Lines` row (#553); written once, and only if it changes.

#### At `item.label(id!(face.marca)).set_text(`

The panel's own TRIANGLE, not the shell's chevron.

The Explorer follows a modern editor and draws a chevron; the Object
Inspector follows the egui screenshot, where the fold mark is the
solid triangle of `CollapsingHeader`. They are two different
references, and `inspector::icon` is what holds this one.

#### At `if matches!(linha, inspector::Row::Prop(_) | inspector::Row::Event(_))`

THE CURRENT ROW is the one that has KEYBOARD FOCUS, and it is QUERIED
from the framework instead of stored in a field.

Deriving instead of storing is what prevents the highlight from lying:
a `linha_corrente: Option<usize>` would have to be cleared when switching
components, switching tabs, and changing the filter -- and forgetting to
clear leaves highlighted what is no longer there. Focus IS the definition of
*"where the keyboard is"*, and what holds it is `Cx`
(`makepad-platform-1.0.0/src/cx_api.rs:584`).

The `Area::Empty` guard is not pedantry: `has_key_focus` compares
`key_focus` against the area, and with nothing focused `key_focus` IS
`Area::Empty` -- without it, every row without an editor would light up at once.

Only a GRID row gets the mark: `Categoria` and `Aviso` are loose
labels, without a `draw_bg` to apply, and `apply_over` on a
property that the template lacks fails SILENTLY, with a log
line (`live_cx.rs:153-166`).

#### At `let alvo = match rascunho {`

The DRAFT takes precedence over the document: while the
user types, it governs the field. See
[`Edicao`] — and the `(linha, slot)` pair is what prevents
`Left` from writing inside the `Right` field.

#### At `let recuo = p.depth as f64 * RECUO_SUB;`

The INDENTATION of a sub-row belongs to the NAME and not to the row, and the reason is the
same that caused a leak in the Explorer row: the value column must
stay aligned with the rows above, otherwise the `Left` field
ends up misaligned with the `Rect` field and the grid ceases to read as a grid.

#### At `let listra = match linha {`

The STRIPE comes from the MODEL, not from the `PortalList` index: the index
counts category headers and fold sub-rows together, and the
count shown in the reference screenshot restarts at each category
— it is written in `PropRow::striped`.

## Inside `escrever_linha`: `let cor = vec4(`

The color comes from the DOCUMENT, so it can only enter via
`apply_over`: the DSL is text that `rustc` never reads, and there is
no way to write a runtime value into it.

## Inside `escrever_linha`: `if let Some(i) = current {`

A value outside the set does NOT become the first item: the combo
would be saying `None` about a document that says something
else. Without an index, it stays where it is and the line does not lie.

## Inside `escrever_linha`: `if caixa.active(cx) != on {`

The comparison is not penny-pinching: `set_active` touches the
animator (`check_box.rs:1689`), and animating on every frame is
the window that never sleeps.

## Inside `escrever_linha`: `inspector::Row::Category { label, open } => {`

THE CATEGORY HEADER: the chevron indicates whether what is underneath
appears, and Rust writes it — the DSL has no way of knowing the
state. It is the same decision as the folding section in the Explorer.

## Inside `escrever_linha`: `inspector::Row::Event(e) => {`

The EVENT row has only one editor — the function name —, so
it does not pass through the cells below.
