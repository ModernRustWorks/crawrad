---
id: ide-app-view-state-ideapp-texto-do-centro
kind: doc
target_file: src/ide_app/view_state.rs
line_hint: 241
author: 
created: 
title: "IdeApp::texto_do_centro"
tags: []
signature: "pub (crate) fn texto_do_centro (& mut self) -> String"
---

The text that the code view shows RIGHT NOW.

Two sources, and they are not the same thing (see [`source`]): the form's
source is **planned at this instant** from the open document,
and the unit is **read from disk** at this instant. Moving a component
changes the former and never the latter — which is exactly why
`Alt+F12` is what the owner named.

#### At `let unidade_do_form = self.host.active_file().map(source::unit_path);`

THE FILE OF THE FRONT TAB COMES FIRST, and it may have nothing to
do with the open form: the Explorer opens any `.rs` in the
project, and for those there is no skeleton to plan nor document to
consult — there is a file on disk.

The form unit still falls into the lower arm, and that is
why the test is for path EQUALITY and not tab existence:
it is `source::user_unit` that knows how to show the skeleton of a
unit that has not yet been generated.
