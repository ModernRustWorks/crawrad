---
id: ide-app-actions-panels-ideapp-handle-panels-actions
kind: doc
target_file: src/ide_app/actions/panels.rs
line_hint: 7
author: 
created: 
title: "IdeApp::handle_panels_actions"
tags: []
signature: "pub (in super :: super) fn handle_panels_actions (& mut self , cx : & mut Cx , actions : & Actions)"
---

Answers the side panels' widgets of a frame.

## Inside `handle_panels_actions`: `let explorador = self`

THE EXPLORER: the clicked row TELLS what it does, and the shell obeys.

**There is no `match` by row type here**, and that is what makes a
new row type cost a single file and nothing more: the row returns an
[`explorer::Activation`], and its four variants are the four
things that a click can be in this shell. Before this, dispatch was
a `match` on the row `enum` ending in `_ => {}` — and a new
type came in silently, with nothing complaining.

The clicked row comes from the SAME list that drew it, via the index that
`PortalList` returns — there is no widget path table to
forget to update, which is the reason the panel is a list.

## Inside `handle_panels_actions`: `let trilho = self.ui.portal_list(id!(activity_rail.rail_list));`

THE ACTIVITY RAIL: clicking brings the panel into view, or collapses it.

**It DOES NOT touch the dock directly: it executes a CATALOG ACTION**,
`layout.toggle_panel`, with this panel's id as an argument. It is the owner's
rule from 15/09/2026 — *"everything in the IDE must have actions, so that the list
can later be used as tools for the MCP"* —, and what it buys
here is concrete: the rail button and the agent tool are the
SAME command, and cannot diverge.

The panel comes from the SAME list that drew the button, via the index that
`PortalList` itself returns — there is no table to forget to
update, which is the reason the rail is a list and not four
buttons written in the DSL.

## Inside `handle_panels_actions`: `let mut fechar = None;`

THE HEADER X of each dockable panel.

It executes `layout.hide_panel` — the same action that the rail's context
menu and the MCP call. Closing a panel by click and closing by
tool are the SAME command, and that is what prevents them from diverging.

## Inside `handle_panels_actions`: `if let Some(frase) = nota {`

AFTER the command, and the order is the point: `execute_with` writes
its own status, and the note is information that only the row has.
If written before, the generic `opened src/form_main.rs` would erase it.

It is the same discipline as `abrir_relativo_ao_membro` in the egui shell
(`main.rs`), which for the same reason writes the error LINE after
the action has spoken.

## Inside `handle_panels_actions`: `let _ = self.abrir_formulario(cx, &caminho);`

The verdict of `abrir_formulario` matters to whatever
RELOADS, so as not to announce "reloaded" for a file
that did not open. Here the phrase was already written to the bar by
the method itself, in both outcomes.

## Inside `handle_panels_actions`: `Some(explorer::Activation::Run(acao, args)) => {`

THROUGH THE CATALOG, entirely. Collapsing a member node and
collapsing the legend are `explorer.toggle_folder` and
`explorer.toggle_legend`, the same ones the protocol calls —
the click is not a second path for the same effect.

## Inside `handle_panels_actions`: `let mut nota = None;`

THE ROW NOTE — what only it knows, and what the action cannot
say. See [`explorer::ExplorerRow::note`].
