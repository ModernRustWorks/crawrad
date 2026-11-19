---
id: ide-app-actions-editor-ideapp-handle-editor-actions
kind: doc
target_file: src/ide_app/actions/editor.rs
line_hint: 1
author:
created:
title: "IdeApp::handle_editor_actions"
tags: []
signature: "pub (in super :: super) fn handle_editor_actions (& mut self , cx : & mut Cx , actions : & Actions)"
---

Answers the code editor's widgets: tabs, find bar and the editor itself.

## Inside `handle_editor_actions`: `let tira = self`

OPEN FILE TABS — clicking opens, clicking `X` closes.

Both buttons come from the SAME item that the list drew, by the index
that `PortalList` itself returns: there is no widget path table to
forget to update, which is the reason the strip is a list.
It is the same design as the Explorer and the rail.

## Inside `handle_editor_actions`: `match self.host.active_file().map(|p| p.to_path_buf()) {`

THIS FORM'S `Code` — not an arbitrary file.

It opens the same tab up there that `F12` opens, through the same
door: it is the point where the two strips meet, and where
they must agree. Two doors for the same unit
would be two tabs on the same file.

## Inside `handle_editor_actions`: `let mut abas_do_centro = WidgetSet::empty();`

THE DESIGN TAB. It returns to the canvas — and it also
undoes `Alt+F12`, because its label says which of the two modes
the design is in (`Design` / `Design Code`).

The two are a `RadioButtonSet` — the same piece as the Object
Inspector tabs, and what unchecks the other is the set itself. FILE
tabs do not enter this group: they are born at runtime
inside a `PortalList`, and a radio group requires members
declared in the DSL (see `CrawradEditorTab`, which therefore keeps the highlight
in the shader).

## Inside `handle_editor_actions`: `if let Some(i) = fechar {`

CLOSE BEFORE OPENING, and the order is not a matter of taste: `X` lives INSIDE
the tab, and a click on it that also counted as a click on the tab
would open the file that was just closed.

## Inside `handle_editor_actions`: `} else if i == 1 {`

Already in Code: nothing to do, as in the classic RAD IDE. Traced on
30/09/2026: a debugger stop's Design -> Code switch lit this
tab, and the selection reopened the form's unit over the stop.

## Inside `handle_editor_actions`: `None => self.show_center(cx, Center::Code),`

Without an open form there is no unit to open, and the code
view remains the one that explains what is missing.

## Inside `handle_editor_actions`: `let welcome = self.host.center() == crawrad_host::ViewCenter::Welcome;`

Under the Welcome page the strip is hidden: a selection from it is
the framework's, never the user's, and it took the centre (#555).
