---
id: canvas-atlas-pictureatlas-pack-shelf
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 68
author: 
created: 
title: "PictureAtlas::pack_shelf"
tags: []
signature: "fn pack_shelf (& mut self , w : usize , h : usize) -> Option < (usize , usize) >"
---

Allocates space on the current shelf or advances to the next shelf.

Returns `None` when the atlas is full.
