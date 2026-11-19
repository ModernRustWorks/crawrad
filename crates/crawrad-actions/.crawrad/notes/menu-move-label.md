---
id: menu-move-label
kind: doc
target_file: src/menu.rs
line_hint: 527
author: 
created: 
title: "move_label"
tags: []
signature: "fn move_label (side : DockSide) -> Option < & 'static str >"
---

The label of `Move To`, one per edge.

The `match` is exhaustive: a new edge in [`DockSide`] does not compile until it earns
text. The text is `&'static str` and not `format!` because [`Row::label`] is
`&'static str` — and it is so that no menu label can be
assembled at runtime from a catalogue id.
