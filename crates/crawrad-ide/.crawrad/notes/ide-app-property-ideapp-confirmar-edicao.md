---
id: ide-app-property-ideapp-confirmar-edicao
kind: doc
target_file: src/ide_app/property.rs
line_hint: 58
author: 
created: 
title: "IdeApp::confirmar_edicao"
tags: []
signature: "pub (super) fn confirmar_edicao (& mut self , cx : & mut Cx)"
---

Commits the Inspector draft, if there is one.

Called on `Enter` and on blur, which are the two moments when
the classic Object Inspector saves.

**The rejection is stated.** Typing `abc` into `Position` and leaving the field
cannot be a silent nothing: the phrase goes to the status bar, and the
field returns to the document value in the next frame because the draft
dies here anyway.

#### At `let Some(editor) = self.editor_da_linha(e.linha) else {`

THE EDITOR is reconstructed from the catalogue, and not stored in the draft.

Storing it cost a `Box<dyn PropEditor>` cloned per keystroke; worse
than that, it aged — a `Ctrl+Z` in the middle of typing changed the
document underneath, and the draft still carried the old value, which
is precisely what `Margins` writes back ENTIRELY. Reassembling
by row uses the same path that drawing used.
