---
id: ide-app-arrangement-ideapp-restaurar-arranjo
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 285
author: 
created: 
title: "IdeApp::restaurar_arranjo"
tags: []
signature: "pub (super) fn restaurar_arranjo (& mut self , cx : & mut Cx)"
---

Restores the dock to the arrangement that the IDE ships with out of the box.

**Mandatory, not decoration** (Amendment 9): an arrangement that persists with no
way back is a trap — the user drags everything to the wrong place once and
has no way to undo it.

The default comes from the DSL, and not from a `HashMap` written in Rust:
`IdeApp::new_from_...` saves the `dock_items` from `live_design!` upon
instantiation, so it is enough to rebuild the entire widget to have them
back — but that would throw away the currently open document along with it. The cheap
and honest path is to reread the DSL from the `Dock` itself: `Self::ARRANJO_PADRAO` is the
same thing written once, in Rust, and there is a test verifying that its IDs
exist in the DSL.
