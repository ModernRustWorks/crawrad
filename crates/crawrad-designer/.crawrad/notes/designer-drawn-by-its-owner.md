---
id: designer-drawn-by-its-owner
kind: doc
target_file: src/lib.rs
line_hint: 404
author: 
created: 
title: "Designer::drawn_by_its_owner"
tags: []
signature: "fn drawn_by_its_owner (& self , doc : & Document , id : & str) -> bool"
---

Returns whether the node is drawn by its visual owner.

An item of its owner's collection on a VISUAL owner is drawn by that
owner (#458: the `StatusBar` paints its `StatusPanel`s), so it has no
place of its own on the canvas. The owner is whoever declares
`Filhos::NaColecao { item }` for the node's class.
