---
id: ide-app-build-ideapp-save-form
kind: doc
target_file: src/ide_app/build.rs
line_hint: 161
author: 
created: 
title: "IdeApp::save_form"
tags: []
signature: "pub (super) fn save_form (& mut self , cx : & mut Cx) -> bool"
---

Saves the form to disk and runs generation.

The form's half of a save: the `.crawform` to disk, then the
generation. `false` when the form could not be written.

#### At `self.reabrir_no_canvas(cx);`

THE DOCUMENT MAY HAVE CHANGED — pruning unlinks the stale binding —,
so the canvas, the Inspector, and the bar are rewritten. Without this, the
name of the pruned handler would remain on the event line until the next
click.

## Inside `save_form`: `self.status = Some(format!("{salvo} — {gerado} (the unlink is not o...`

The `.crawform` fell behind, and saying so is all that belongs
here: what is on screen is correct, and the user needs
to know that the disk did not keep up.

## Inside `save_form`: `let salvo = self.status.take().unwrap_or_default();`

BOTH SENTENCES on the bar, and not just the one from generation: what the user requested
was to save, and the response to that is the first half. The second is
what came along with it — including pruning, which is the reason for this line.

## Inside `save_form`: `if self.host.doc().map(|d| d.revision()) != antes`

The pruning changed the document after it was saved, so the disk is
rewritten once (no new generation: that would never end).
