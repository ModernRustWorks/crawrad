---
id: ide-app-dialogs-notes-ideapp-paint-note-card
kind: doc
target_file: src/ide_app/dialogs/notes.rs
line_hint: 228
author: 
created: 
title: "IdeApp::paint_note_card"
tags: []
signature: "pub (in super :: super) fn paint_note_card (& mut self , cx : & mut Cx , card : & ViewRef , id : & str , markdown : & str , line_height : f64 ,) -> f64"
---

Paints the documentation note hover card.

A noted item's card: rust-analyzer's markdown (the note is the item's
doc), the kind and the way to open it; the card's height.
