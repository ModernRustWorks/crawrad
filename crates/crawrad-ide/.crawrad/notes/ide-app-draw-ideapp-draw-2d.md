---
id: ide-app-draw-ideapp-draw-2d
kind: doc
target_file: src/ide_app/draw.rs
line_hint: 1
author:
created:
title: "IdeApp::draw_2d"
tags: []
signature: "pub (super) fn draw_2d (& mut self , cx : & mut Cx2d)"
---

Draws the window, filling each `PortalList` as the draw step reaches it.

## Inside `draw_2d`: `if let Some(eventos) = rects_dos_eventos {`

The Inspector one replaces even if EMPTY, and the difference is in
its declaration up above: empty here means the panel drew and there
was no event at all.

## Inside `draw_2d`: `if !rail_rects.is_empty() {`

The rail might not have been drawn in this frame (it only comes
during the `PortalList` step). Storing an EMPTY vector over the valid
one would wipe out the right-click hit-test until its next draw.

## Inside `draw_2d`: `let kind = if linha.nivel == 0 {`

The ROOT is the form; the rest comes from the catalog. A node
without a class (none exists today) is left without an icon
instead of getting a random one.

## Inside `draw_2d`: `let recuo = RECUO_BASE + RECUO_POR_NIVEL * linha.nivel as f64;`

The indentation is PIXELS, not whitespace in text — the same
decision (and the same number) as egui's tree
(`tree.rs:127-140`). `apply_over` with `live!` is the official
path to change a property at runtime without touching DSL
text (`widget.rs:733`). It goes on `face`, not on the row:
it is the one that holds the content, and the click target
keeps the full width.

## Inside `draw_2d`: `item.label(id!(face.visto)).set_text(`

The CHECKMARK — the mark of the selected row, and it was the owner
who chose it: *"I preferred a Checkmark (ticket) on what is
selected instead of selecting this way"*.

The glyph comes from the icon catalog (ADR 0026), and the role is
the same that the menu already uses to convey state.

## Inside `draw_2d`: `let molde = if linha.nivel == 0 {`

TWO templates, and the question is just one: is it the root? Its
name comes in bold, because the form is not a component among
the others — it is the owner of all of them.

SELECTION DOES NOT CHOOSE A TEMPLATE, and this is the fix from
2026-09-17. By choosing, it changed the row's template at
runtime — and a changed template makes `PortalList` build
ANOTHER widget for that index, the very one that had just
emitted the click. The owner saw the result: *"clicking in
Structure should select the designer component, but it isn't"*.

## Inside `draw_2d`: `if let Some(mut lista) = explorador.borrow_mut_if_eq(&proximo) {`

The EXPLORER. The rows are assembled IN HERE, and only when the
panel is drawn: assembling before the loop would cost the generator
plan and a disk scan every frame, with the panel closed — and it
starts closed for anyone who has never opened it.

## Inside `draw_2d`: `_ => item.set_text(cx, linha.text()),`

`WidgetRef::set_text` (`widget.rs:315`) handles the section
label through the same call: it goes down via the `Widget`
trait, which `Label` implements.

## Inside `draw_2d`: `palette::Row::Item { class, label } => {`

The component row has an ICON, and because of this it is no
longer a text `Button`: the `set_text` of a `View` does
nothing (the default implementation of the `Widget` trait is
empty — `widget.rs:36`), and without this the Toolbox would
have come out blank, silently.

## Inside `draw_2d`: `let marca = f64::from(u8::from(`

The HIGHLIGHT of the selected item is the `active` state of the
same button that receives the click, as on the rail — and not a
second template, which would recreate the widget on each change.

## Inside `draw_2d`: `let marca = f64::from(u8::from(act_sel.as_deref() == Some(&linha.id...`

The HIGHLIGHT is the selection of the DOCUMENT, and not a state
of this window: clicking an action here selects the component, and
that is what makes the Object Inspector behind show its
properties — as in a classic RAD IDE.

## Inside `draw_2d`: `let marca = f64::from(u8::from(iml_sel == Some(indice)));`

The HIGHLIGHT of the chosen one is the BACKGROUND of the cell,
as in egui: the checkerboard is content and cannot carry state.
The colors are from the theme — see `CrawradIconCell`.

## Inside `draw_2d`: `if let Some(mut lista) = iml_grade.borrow_mut_if_eq(&proximo) {`

The IMAGE GRID: a row of six per list item. The leftover cells
in the last row are HIDDEN, not drawn empty — an empty cell
with a checkerboard inside looks like a transparent image, which
is the opposite of "there is no image here".

## Inside `draw_2d`: `let (item, existia) = lista.item_with_existed(cx, i, molde);`

The `existed` flag is what tells if the widget was born JUST
NOW (`portal_list.rs:563`), and on it depends the combos
cache — see the `enums_na_tela` field.

## Inside `draw_2d`: `let molde = match linha {`

The TEMPLATE NAME COMES FROM THE EDITOR ITSELF
(`PropEditor::template`), not from a `match` here. That is what
makes a new property type cost a file in
`ide_app/inspector/editors/` and zero lines in this shell — and
it is the same path that the Explorer already uses.

## Inside `draw_2d`: `let marca = f64::from(u8::from(aba_a_frente == Some(i)));`

The HIGHLIGHT IS A SHADER `instance`, and not a second
template: the front tab changes with each click, and changing
templates would recreate the widget that `PortalList` just
recycled. It is the same grammar as the rail.

## Inside `draw_2d`: `let marca = f64::from(u8::from(self.painel_a_vista(&arranjo, painel...`

The accent bar of the active item is an `instance` of the button
shader (`tema.rs`, `CrawradRailButton`), not a second template:
the state changes with each click, and changing templates would
recreate the widget.

## Inside `draw_2d`: `let mut rects_dos_eventos: Option<Vec<(usize, Rect)>> = None;`

`Option` and not `Vec`: the `Properties` tab draws the SAME
`PortalList` without any event row, and an empty `Vec` would
not distinguish *"the Inspector drew and there is no event"* from
*"the Inspector did not draw in this step"*. Without the distinction,
switching to `Properties` would leave the rectangles from the
previous tab active — and double-clicking a property would open
the handler of the event that used to occupy that row.

## Inside `draw_2d`: `let explorador = self`

THE TWO LISTS OF THE NEW PANELS. They live INSIDE the dock, so
the path starts at the tab and not in `ui`: `Dock::item` returns
the panel's widget, and the `PortalList` is inside there. A closed
tab returns an empty `WidgetRef`, and `borrow_mut_if_eq` simply
does not match.

## Inside `draw_2d`: `let arranjo = self.arranjo();`

The dock LAYOUT, read ONCE per frame: from it comes the rail's
accent bar, and it is the only source — there is no field stating
which panel is open.

## Inside `draw_2d`: `let alheios = crawrad_host::navigation::foreign_roots();`

The DIRTY TAB DOT goes in HERE and not in the model: `editor_tabs`
is neutral on purpose — it does not know what a disk is, and what
it resolves is name disambiguation. The one that knows what is yet
to be saved is this shell, which holds the buffer (see [`editor_buffer`]).

## Inside `draw_2d`: `let abas_do_editor = self`

The editor TAB STRIP — one tab per open `.rs`. The labels come
from the model ALREADY disambiguated (`editor_tabs::labels`), because
deciding that inside the draw step would mean deciding it once per
frame and without tests.

## Inside `draw_2d`: `let trilho = self.ui.portal_list(id!(activity_rail.rail_list));`

The RAIL is the fourth list, and it comes from `rail::Destino::TODOS`
for the same reason as the other three: the destination list lives
in Rust, and the DSL only stores a button template.

## Inside `draw_2d`: `let toolbox = self`

The THREE lists of the left panel — Toolbox, Structure, and
Object Inspector — request population through the same draw
step, and `borrow_mut_if_eq` is what separates them: it only
borrows when the step's `WidgetRef` is that one, via `Rc::ptr_eq`
(`widget.rs:352-355`). The method is generated by `#[derive(Widget)]`
(`makepad-derive-widget-1.0.0/src/derive_widget.rs:299-305`).

## Inside `draw_2d`: `if let Some(mut lista) = popup.borrow_mut_if_eq(&proximo) {`

The OPEN MENU. It comes last among the lists because it is the
only one that spends almost every frame empty.

## Inside `draw_2d`: `lista.set_item_range(cx, 0, galeria.len());`

The GALLERY, not the factory table: what the user added
enters here through the same path as the others.

## Inside `draw_2d`: `if matches!(linha, inspector::Row::Event(_)) {`

The rectangle of each EVENT row, saved for double-clicking —
see the `rects_dos_eventos` field.

## Inside `draw_2d`: `let dirty = f64::from(u8::from(dirty_tabs.get(i) == Some(&true)));`

The `X` COMES FROM THE ICON CATALOG, not from the DSL: the
codepoint lives in only one place (ADR 0026).

## Inside `draw_2d`: `rail_rects.push((painel, item.area().rect(cx)));`

The rectangle of EACH button, saved for right-clicking —
see the `rail_rects` field.

## Inside `draw_2d`: `let mut explorer_rows: Option<Vec<explorer::Row>> = None;`

The rows of both are ASSEMBLED ON DEMAND, inside the list draw
step — see the two arms further down.

## Inside `draw_2d`: `let popup = self.ui.portal_list(id!(menu_popup.menu_list));`

The OPEN MENU is the fifth list, and for the same reason as the
others: the number of rows is the number of actions in the group,
which the DSL does not know.

## Inside `draw_2d`: `let items_lista = self`

The FIFTH list: the Items Editor tree. It lives on a body layer,
not on a dock panel — which is why the path starts from `ui`.
