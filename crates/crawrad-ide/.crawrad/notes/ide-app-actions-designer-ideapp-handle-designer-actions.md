---
id: ide-app-actions-designer-ideapp-handle-designer-actions
kind: doc
target_file: src/ide_app/actions/designer.rs
line_hint: 73
author: 
created: 
title: "IdeApp::handle_designer_actions"
tags: []
signature: "pub (in super :: super) fn handle_designer_actions (& mut self , cx : & mut Cx , actions : & Actions)"
---

Answers the designer bar, the canvas and the Inspector widgets of a frame.

## Inside `handle_designer_actions`: `if let Some(inspector::Row::Category { label, .. }) = props.get(i) {`

THE CATEGORY HEADER, which collapses. The entire row is the
target, not just the chevron: this is what a modern editor does with the
section header, and aiming for an 18-pixel glyph is the
opposite of usability.

Through the CATALOG, like everything in this shell: what closes `Layout` is
the `inspector.toggle_category` action, and its id lives in the
panel module — never written here.

## Inside `handle_designer_actions`: `cx.start_dragging(vec![DragItem::FilePath {`

The `internal_id` is what separates OUR drag from a file
dropped from outside: without it, dropping a `.txt` from the
desktop onto the form would attempt to create a component of
class `C:\...`.
`FilePath` and not `String`, and the reason is a FRAMEWORK
LIMITATION on Windows, measured in source: win32's `start_dragging`
only builds the `IDataObject` for `DragItem::FilePath`
— any other variant falls into
`error!("Only DragItem::FilePath supported")`
(`makepad-platform-1.0.0/src/os/windows/win32_app.rs:458`).
With `String`, the drag simply did not happen: nothing on
screen, nothing in the application log.

It is not a workaround: it is Makepad's own path for INTERNAL
drag. What separates "this is mine" from a file dropped from the
outside is `internal_id`, not the variant — which is why
`classe_arrastada` requires both.

## Inside `handle_designer_actions`: `if alvo.pressed(actions) {`

DRAG begins on DOWN, not on click.

In a classic RAD IDE the palette has BOTH gestures, and the owner compared
the two side by side: clicking arms the tool and the next
click on the form releases it; DRAGGING carries the component to
the point and drops it there. The two coexist because a drag that
ends without moving drops nothing, and `clicked`
happens just the same.

`pressed` is the only possible hook: Makepad's `Button`
captures the finger, so `FingerMove` does not reach here — and
`Released` (`button.rs:572`) only notifies AFTER releasing, which
is too late to start a drag.

## Inside `handle_designer_actions`: `if alvo.pressed(actions) {`

**ARMING AND DRAGGING COME FROM THE SAME `pressed`, and this is not
laziness: `clicked` NEVER happens on this line.**

Measured in the framework source on 18/09/2026. The `pressed` of
`FingerDown` makes this shell call `cx.start_dragging`, and on
Windows `CxOsOp::StartDragging` calls SYNCHRONOUS `DoDragDrop`
(`makepad-platform-1.0.0/src/os/windows/win32_app.rs:450`)
— a modal OLE loop that consumes the button until release. The
`FingerUp` never returns to the `Button`, so
`ButtonAction::Clicked` is never emitted.

The consequence, and it was the CENTRAL gesture of a classic RAD IDE:
clicking the palette armed NOTHING. Measured on screen, with
native and third-party components: the Toolbox said *"click a
component, then click the form"* and clicking did nothing.

Arming on DOWN provides the two gestures that a classic RAD IDE has:

- releasing without moving -> the drag cancels, the tool
  remains armed, and the next click on the form releases it;
- dragging to the form -> the drop creates it there, and
  `DragHit::Drop` DISARMS the tool, so that the
  subsequent click does not create a second component that nobody requested.

## Inside `handle_designer_actions`: `let campo = self`

THE TOOLBOX `Filter` FIELD

It existed in the DSL and was read by no one: the user typed and
the list did not move. `TextInputRef::changed` returns the new text on
each change (`makepad-widgets-1.0.0/src/text_input.rs:1843`), and it is
the only channel — `TextInput` does not expose the text to the drawing of another
widget.

There is no `set_first_id(0)` here, and its absence was MEASURED.
`PortalList` keeps the top index (`portal_list.rs:82`) and does not
reduce it when the range shrinks outside of `tail_range` mode
(`portal_list.rs:648-657`), so the suspicion was that filtering with a
scrolled list would leave the panel blank. It does not: with the Toolbox
scrolled down to `System / Timer` and the filter dropping to 7, 5, and 2 rows,
all three times the list returned to the top by itself — `next_visible_item`
itself recedes when reaching the end with slack
(`portal_list.rs:405-430`). A call that changes nothing on screen is an
assertion, not code.

## Inside `handle_designer_actions`: `if self`

THE COLLECTION `+`, in the Structure header.

It sends `component.add_item` with the selected node, and **does not know
any class name**: what determines which class the item belongs to is the
parent's descriptor (ADR 0037). A third-party grid declaring
`item_class = "Column"` in `manifest.json` gains this button without a
single line written here.

Without it there was no path: `Action` has `palette: ""` on purpose
and the only gesture that created a node was dropping from the palette — measured on
18/09/2026, the user had no way to create an `Action`.

## Inside `handle_designer_actions`: `let lista = self`

THE STRUCTURE TREE: the clicked row selects its node.

The index comes from `PortalList` itself (`items_with_actions`), and the
list of rows is reconstructed from the document — the same one that drawing
used, through the same function.

## Inside `handle_designer_actions`: `if campo.escaped(actions) {`

`Esc` abandons editing, as in a classic RAD IDE: the field
reverts to the document's value in the next frame,
because without a draft what controls the field is the
drawing.

## Inside `handle_designer_actions`: `if let Some(texto) = campo.changed(actions) {`

Typing DOES NOT save: it only updates the draft. The
reason is in [`Edicao`] — there are two measured
reasons, and the second is `Ctrl+Z` undoing the
entire word instead of a single letter.

## Inside `handle_designer_actions`: `for celula in p.editor.cells() {`

THERE IS NO `match` BY EDITOR TYPE HERE either: the editor
declared which slots it has, and the loop only asks each of these
widgets if it was touched. A new property type does not
add an arm to this function.

## Inside `handle_designer_actions`: `let inspetor = self`

THE OBJECT INSPECTOR: the editor for each row, by its TYPE.

The index comes from `PortalList` itself (`items_with_actions`), and the
list of rows is reconstructed from the catalog — the same one that drawing
used, through the same function. There is no widget table to keep up to date:
adding a property to `crawrad-registry` enters here on its own.

## Inside `handle_designer_actions`: `let toolbox = self`

Toolbox Arming Actions

The class comes from the SAME list that drew the row, via the index that
`PortalList` itself returns — there is no table to forget to
update. A section label arms nothing because it is not a
`Button`: `ButtonRef::clicked` borrows the widget
(`button.rs:819-821`) and a `Label` returns `None`.

## Inside `handle_designer_actions`: `self.redraw_panel_list(cx, Panel::Inspector, id!(inspector_panel.in...`

THE LIST, and not just the `ui`: the tab would light up and the grid
kept showing the properties — measured on screen on
16/09/2026. It is the same trap as the canvas and panel
chrome: `self.ui.redraw` does not reach a `PortalList`
residing inside a dock item
([`IdeApp::redraw_panel_list`]).

## Inside `handle_designer_actions`: `self.edicao = None;`

Switching tabs DISCARDS what was being typed: the
field leaves the screen, and saving over a document that
the user is no longer looking at is the change nobody
asked for.

## Inside `handle_designer_actions`: `let mut abas = WidgetSet::empty();`

THE OBJECT INSPECTOR TABS — `Properties` and `Events`.

The set is assembled from [`InspectorTab::ALL`], not from a list
of paths written here: this way the enum ORDER and the group order
cannot diverge. What unchecks the other tab is `RadioButtonSet`
itself (`radio_button.rs:1420-1433`).

## Inside `handle_designer_actions`: `inspector::Cell::Caption { .. } | inspector::Cell::Swatch { .. } => {}`

A label and a color swatch receive no gestures:
they DISPLAY. What edits the color is the field
next to the swatch, which is a `Field`.

## Inside `handle_designer_actions`: `let mut pedido: Option<(inspector::Outcome, NodeId, &'static str)> ...`

What the row requested, decided BEFORE modifying the document: the
loop holds `props`, and acting inside it would borrow the document
twice.

## Inside `handle_designer_actions`: `if campo.escaped(actions) && !self.filtro.is_empty() {`

`Esc` clears the filter. That is what `✕` does in egui's Toolbox
(`crates/crawrad-ide/src/paleta.rs:464-467`), without taking up a click
target in the narrowest column on the screen.

## Inside `handle_designer_actions`: `if let Some(inspector::Row::Event(e)) = props.get(i) {`

THE `Events` TAB: the field is a function name, and what saves it is
the `event.bind` action — see [`IdeApp::confirm_event`].
