---
id: ide-app-maximize-ideapp-toggle-maximized
kind: doc
target_file: src/ide_app/maximize.rs
line_hint: 77
author: 
created: 
title: "IdeApp::toggle_maximized"
tags: []
signature: "pub (super) fn toggle_maximized (& mut self , cx : & mut Cx)"
---

Maximizes or restores the IDE window to its previous position.

**Maximize, or put the window back where it was.**

Both halves are one method because they are one button: the pair
`btn_max`/`btn_restore` shows whichever describes the NEXT click, and
`self.maximizada` is the only thing that knows which — see the field's
own doc for why the framework cannot be asked.

# The fallback, declared

When the adapter has no answer — every platform but Windows, a Windows
that refused to describe its own monitor, or a process that does not
have exactly one window to move — this falls back to
`CxOsOp::MaximizeWindow`. That is the right call and not a surrender:
macOS and Linux maximize to the usable area on their own, so the
framework's op is already correct there. `restore_to` is cleared in
that case, because the framework's `RestoreWindow` is the only thing
that can undo the framework's own maximize.

#### At `let antes = on_screen(&geom);`

THE PREVIOUS RECTANGLE is read NOW, and stored only if movement
happens: storing it first would leave a `restore_to` pointing
to a window that never moved, and the next click would
"restore" it to where it already is.
