---
id: ide-app-explorer-actions-ideapp-aplicar-dobra-do-explorer
kind: doc
target_file: src/ide_app/explorer_actions.rs
line_hint: 343
author: 
created: 
title: "IdeApp::aplicar_dobra_do_explorer"
tags: []
signature: "pub (super) fn aplicar_dobra_do_explorer (& mut self , cx : & mut Cx , comando : crawrad_actions :: ExplorerCommand ,)"
---

Applies an Explorer fold command, from a click or from the protocol.

**THE LIST NEEDS ITS OWN REDRAW**, and this was MEASURED on screen on
2026-09-16: with only `self.ui.redraw(cx)` the set of folds changed (the
state was correct) and the tree remained open. A `PortalList` that lives
inside a `Dock` item is not reached by the window redraw — the `Dock`
draws its children on its own (`dock.rs:185`, `find_widgets` only
descends through items that exist) — and the symptom is the worst kind:
the screen displays the prior state, with nothing in the log.
