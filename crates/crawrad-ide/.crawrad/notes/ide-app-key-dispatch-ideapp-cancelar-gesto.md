---
id: ide-app-key-dispatch-ideapp-cancelar-gesto
kind: doc
target_file: src/ide_app/key_dispatch.rs
line_hint: 149
author: 
created: 
title: "IdeApp::cancelar_gesto"
tags: []
signature: "pub (super) fn cancelar_gesto (& mut self , cx : & mut Cx)"
---

`Esc`: undo the ongoing gesture.

# Text field precedence, written where it lives

`Esc` already had an owner in this shell: it clears the Toolbox `Filter` field
(see `campo.escaped(actions)` in [`IdeApp::handle_actions`]). The two
do not clash, and not by chance: with the field focused, `TextInput`
consumes the key and emits `TextInputAction::Escaped`
(`makepad-widgets-1.0.0/src/text_input.rs:1556`) instead of
`KeyDownUnhandled` — so `Esc` **never reaches here** while
typing. Outside the field, [`IdeApp::handle_key_down`] brings the key
directly. Whichever has focus wins, and the rule belongs to the framework, not
ours.
