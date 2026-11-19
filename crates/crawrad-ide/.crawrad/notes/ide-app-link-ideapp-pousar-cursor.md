---
id: ide-app-link-ideapp-pousar-cursor
kind: doc
target_file: src/ide_app/link.rs
line_hint: 204
author: 
created: 
title: "IdeApp::pousar_cursor"
tags: []
signature: "pub (super) fn pousar_cursor (& mut self , cx : & mut Cx , pouso : & crate :: ide_app :: PousoCodigo)"
---

Puts the code view cursor inside the handler body.

# This is the gesture, not decoration

ADR 0015 says that the editor exists because of the LINK, and the link does not end at
*showing the file*: in a classic RAD IDE — and in the egui shell — double-clicking lands
the cursor **inside the body**, ready to type, even when the block
already existed. Reaching the right line and leaving the cursor at the beginning of the file is stopping one step short of the end.

# What was written here, and was wrong

*"This shell's view is a READ-ONLY `CodeView`: line and column go
to the status bar, and there is no cursor to land."* It was deduction, not
measurement. `CodeView` exposes `session` and `editor`
(`makepad-code-editor-1.0.0/src/code_view.rs:30-33`), `CodeSession`
has `set_selection` (`session.rs:197`), and `CodeEditor` has
`keep_cursor_in_view` (`code_editor.rs:237`), which is what SCROLLS there. Read-only prevents typing, not positioning.

# Order matters

`set_text` on `CodeView` **discards the session** (`code_view.rs:82`), and
it is only reborn in the following drawing. Therefore `lazy_init_session` is
called here before anything else: without it, this function would touch a session that
does not exist and the cursor would remain where it was.
# Why it happens in the FOLLOWING frame

Focus needs the editor's AREA, and the area only exists after it has
drawn (`Area::Empty` until then). At the moment of the double-click, the code
view just became visible and the text was just changed — there is no
area to focus, and a `set_key_focus(Area::Empty)` does nothing.
Measured: the cursor went to the right place and remained INVISIBLE.

## Inside `pousar_cursor`: `cv.editor.reveal_cursor(cx);`

AND SCROLLS THERE. Without this, the cursor goes to the right line of a file
that keeps showing the top — the new handler is born at the end, and the end
is precisely what is not in view.
