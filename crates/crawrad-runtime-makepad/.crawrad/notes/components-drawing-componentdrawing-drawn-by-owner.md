---
id: components-drawing-componentdrawing-drawn-by-owner
kind: doc
target_file: src/components/drawing.rs
line_hint: 87
author: 
created: 
title: "ComponentDrawing::drawn_by_owner"
tags: []
signature: "fn drawn_by_owner (& self) -> Option < & 'static str >"
---

The owner class that draws this item, or None if it draws itself.

The class that DRAWS this one, when it is an item of its owner's
collection and has no face of its own on the canvas (#458: a
`StatusPanel` is drawn by its `StatusBar`). `None`: it draws itself.
