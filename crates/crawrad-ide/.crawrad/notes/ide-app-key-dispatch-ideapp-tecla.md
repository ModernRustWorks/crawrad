---
id: ide-app-key-dispatch-ideapp-tecla
kind: doc
target_file: src/ide_app/key_dispatch.rs
line_hint: 1
author:
created:
title: "IdeApp::tecla"
tags: []
signature: "pub (super) fn tecla (& mut self , cx : & mut Cx , ke : & KeyEvent , do_campo : bool)"
---

Routes one key press: the shortcuts of the catalogue, then the designer.

## Inside `tecla`: `teclado::Comando::Acao(id) => self.execute_action(cx, id),`

Through the catalogue, and entirely: `habilitada` rejects, `executar`
returns the `Efeito`, and `apply_effect` applies it. No
transaction is born here (ADR 0005).

## Inside `tecla`: `let teclado_do_texto = self.editor_com_foco(cx);`

**AND THE EDITING KEYBOARD BELONGS TO THE EDITOR, when it has focus.**

`Ctrl+Z` with the cursor in a file would undo the last transaction of the
FORM, and `Del` would delete the selected component. See
[`crawrad_actions::key_routing::e_do_editor_de_texto`] for the rule and measure.

## Inside `tecla`: `let no_editor = self.host.center().is_text();`

**THE DESIGNER GESTURE DOES NOT CROSS THE EDITOR.**

The arrow keys and `Esc` are canvas gestures — nudge the component,
cancel the drag —, and with the code view in front they have nothing
to nudge: what the user is asking is to move the CURSOR, and
`CodeEditor` knows how to do that (`code_editor.rs:745` and `:784`).

Without this guard, both happen in the same keypress: the cursor moves and the
selected component moves along, behind the code, without anyone seeing.
The owner noticed via the visible half — *"arrow [does not work]"* —, and
the half that does not appear is worse.

The catalogue ACTIONS keep passing through: `Ctrl+S`, `F12`, and the rest are
commands of the IDE, and the IDE does not cease to exist because the editor is
open.

## Inside `tecla`: `self.status = None;`

Message is TRANSIENT, as in mouse gestures (see `bombear`): without
this, the rejection of a `Del` without a selection would stay on the bar forever.
