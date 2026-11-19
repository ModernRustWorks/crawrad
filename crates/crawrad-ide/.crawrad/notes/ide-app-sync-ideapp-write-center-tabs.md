---
id: ide-app-sync-ideapp-write-center-tabs
kind: doc
target_file: src/ide_app/sync.rs
line_hint: 254
author: 
created: 
title: "IdeApp::write_center_tabs"
tags: []
signature: "pub (super) fn write_center_tabs (& mut self , cx : & mut Cx)"
---

Places a view in the center — **and rewrites the text upon entering it**.

The rewriting is the entire function, and is the owner's request from 09/15/2026:
*"when switching to code design it must trigger the generator, to have
up-to-date code"*. Without it the `Code` tab of this shell was an EMPTY text
box — `toggle_view` flipped two `visible` flags and wrote nothing into
`code_view`.

Showing derived text without re-deriving it has bitten this codebase before: it is
what `pousar_na_linha` in the egui shell (`main.rs:3744-3750`) documents —
*"the `.designer.rs` on screen is the one from before generation, and the error lines
are those from after"*.
The TWO LABELS of the center strip, and which of the two tabs is lit.

# Why it is its own function, and not part of `show_center`

Because it has TWO callers at different times, and the second was
missing. `show_center` only runs when the view CHANGES — at startup
nothing changes, so the strip started **blank**: two tabs without
text and neither lit. Verified via screenshot: the strip existed, the canvas
drew correctly, and a click anywhere on it took you to the code
view without anything indicating why.

And the second caller is `escrever_cromo_dos_paineis`, which runs AFTER
dock's `load_state` — for the same reason the
`Properties | Events` strip needed it: `Dock` recreates each
item's widget, and whatever was written before is lost without any error.

#### At `let acesa = match centro {`

The lit tab follows the centre (`FormSource` lights Design, renamed
`Design Code`), and EVERY other one is switched off by hand: a
`RadioButton` only emits `Clicked` from `active.off`
(`makepad-widgets-1.0.0/src/radio_button.rs:1361-1365`) and its
`unselect` is private (`:1402`), so a tab left on goes dead (17/09/2026).

#### At `self.painel(Panel::Design)`

NO BULLET. The `• Code` was the patch from when both were `Button`
and the highlight did not move: the marker remained in the TEXT because there
was no way to move it in the BACKGROUND. Now what tells which one is open is the
tab surface, as in any tab strip.

## Inside `write_center_tabs`: `if centro != Center::Welcome {`

Selecting EMITS a click, handled as one: under the Welcome page (its
strip hidden) it would switch the centre back to Design (#555).
