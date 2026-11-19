---
id: ide-app-effects-ideapp-apply-effect
kind: doc
target_file: src/ide_app/effects.rs
line_hint: 76
author: 
created: 
title: "IdeApp::apply_effect"
tags: []
signature: "pub fn apply_effect (& mut self , cx : & mut Cx , efeito : crawrad_actions :: Effect ,) -> Result < () , String >"
---

Applies the declarative effect produced by an executed action.

What this shell does **not** do, it states. Previously, `Undo`,
`Redo`, and `Save` called the action, received the effect, and
discarded everything that wasn't a transaction: the three buttons
did nothing, silently.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
assert!(app.status.is_none());
```

# Errors

Returns an error if a transaction fails to apply to the document, or if undo/redo fails.

#### At `let depois = self`

**DID THE DOCUMENT CHANGE? THEN THE SCREEN LEARNS.**

#### The defect that this line closes, and it was of an entire class

This port applied the transaction and left. The CALLER was the one
repainting — the menu, the toolbar, and the Inspector called
`reabrir_no_canvas` + `sincronizar` right after —, and whoever
forgot was left with a modified document and an unchanged screen.

That is what happened with PALETTE DRAG-AND-DROP. Measured with a
probe on 2026-09-17: the `Drop` arrived, the class and point were
correct, `component.add` ran and the document went to `rev=1` —
and the bar kept saying `20 nodes  rev 0`, with the canvas missing
the component. It wasn't dragging that didn't work; it was the screen
that didn't know.

Here, and not in each caller, because here is where it can be
known: the before-snapshot is in this function. A fifth caller is
born covered.

#### At `if let Some(texto) = efeito.copy_text {`

WHAT WAS COPIED.

Two ports for the same string: the IDE's own clipboard, which is
where `edit.paste` pastes from, and the system clipboard, so that
the clipped content crosses over to another window. Without the
first, `Ctrl+V` would answer "clipboard is empty" right after a
`Ctrl+C`.

#### At `if let Some(pedido) = efeito.ui_zoom {`

CANVAS ZOOM.

It was in `Efeito` and FELL TO THE FLOOR: `grep -n zoom src/ide_app.rs`
returned empty. With the keyboard bridge connected, `Ctrl+Shift+.`
would execute the action, return the effect, and not change a single
pixel — the silent bug that this shell already had with `Historico`
and `Arquivo`. The step calculation is performed by `crawrad-runtime`
(`zoom_vizinho`), not this shell.
THE INTERFACE ZOOM — and it is not the canvas zoom right below.

#### At `if let Some(comando) = efeito.layout {`

The VIEW and the DESIGNER MODE — the two families that remained.

They fell to the floor along with zoom, and with the keyboard bridge
connected that turns into `F12` executing the action and the screen
staying unchanged. What is wired is wired; what isn't SAYS it isn't
— that is the rule of this shell since it stopped telling the three
fictions.
The PANEL LAYOUT (ADR 0014, Amendment 9). It is the ONLY port:
the rail, the `View` menu, and the MCP all arrive here, because
they all go through the same catalog action.

#### At `Some(crawrad_actions::FileEffect::OpenInEditor(_)) => {`

The ONLY one that remains unwired, and it states its own name
instead of inheriting the excuse of a `_`. What is missing is not
the gesture of launching — [`crawrad_os::process::detach`] is the
same on both sides — it is the path validation of `abrir_no_editor`
from the egui shell (`main.rs:3783`), which resolves links and
rejects what goes outside the project. Copying it would be writing
the security rule twice, and that is exactly how one of the two
copies goes stale.

#### At `match efeito.file {`

The `Arquivo` FAMILY, VARIANT BY VARIANT — and the `Some(_)` that
was here is the entire issue #147.

It read `efeito.arquivo`, handled `Salvar`, and sent the other
FIVE to an excuse message. The owner's `Run` landed there: the
button dispatched, the action returned `Arquivo::Rodar`, and the
IDE replied *"this action is not wired in the Makepad shell yet"*.
The coverage gate for `Efeito` stayed green because it measured
FIELDS, and `rustc` stayed silent because `Some(_)` is exhaustive
for it — silence on two layers, which is what issue #133 came to
eliminate.

Today the gate requires VARIANTS: deleting any branch here fails
in `crawrad-actions`, and a new `Arquivo` variant is born requiring
both shells.

#### At `self.status = Some(`

The command has NO visible response at the instant it runs — nothing
moves, nothing appears, nothing changes color. Without this line,
turning the lock on and off are the exact same nothing on the screen.
It is the same phrase as in the egui shell (`main.rs:4730-4734`).

#### At `Some(crawrad_actions::ViewEffect::ToggleFormSource) => self.toggle_form_s`

The `Alt+F12` — the form as text, instead of the drawing. It used
to fall into `Some(_)` and only say it wasn't wired; now it is, and
that is what the owner specified: *"when switching to code design
it must trigger the generator, to have updated code"*.

#### At `match efeito.debug {`

The COMPONENT SUITE — the `Component` of a classic RAD IDE, and
step §9 of ADR 0033. Variant by variant, like the `Arquivo` family
right above: a `Some(_)` here would be the blank check that issue
#147 cost.

## Inside `apply_effect`: `if let Some(comando) = efeito.inspector {`

THE TWO WINDOWS AND FOLDS OF THE OBJECT INSPECTOR. Same rule: this
is the ONLY port, and through it arrive the click on `…`, the click
on the chevron, and the MCP.

## Inside `apply_effect`: `if let Some(comando) = efeito.explorer {`

THE EXPLORER FOLDS. Same rule as the layout, and it is the ONLY
port: the click on the chevron, the click on the legend header,
and the MCP all arrive here, because they all pass through the same
catalog action.

## Inside `apply_effect`: `Some(crawrad_actions::FileEffect::OpenInTab(rel)) => self.abrir_aba_re...`

AND THIS EDITOR'S TAB, which is the opposite of the one above: this
shell HAS a tab strip (see `crate::view_models::editor_tabs`), so
the file opens inside the IDE, alongside the design.

## Inside `apply_effect`: `if let Some(crawrad_actions::DesignerMode::ToggleLive) = efeito.design...`

The Preview is a dry view: the form as the product shows it, without
ruler, handles or grid, and nothing in it answers — that is Run.

## Inside `apply_effect`: `let antes = self`

The BEFORE-SNAPSHOT, to know at the end if anything changed — see
the end of this function.
