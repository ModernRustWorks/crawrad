---
id: designer-gesture-permitted
kind: doc
target_file: src/lib.rs
line_hint: 333
author: 
created: 
title: "Designer::gesture_permitted"
tags: []
signature: "fn gesture_permitted (& self , doc : & Document , d : & Drag) -> bool"
---

Returns whether the lock permits this gesture to start.

Does the lock let this gesture start?

It is not the gate — it is what prevents a PHANTOM drag. Without this the gesture
would start, draw guideline and resize cursor, and only not move
anything because [`Designer::refuses`] bars the transaction ahead: the
screen would show a drag happening and the document would not change.

Exhaustive `match` for the same reason as the gate.
