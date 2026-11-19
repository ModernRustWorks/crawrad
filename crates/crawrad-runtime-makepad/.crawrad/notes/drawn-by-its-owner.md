---
id: drawn-by-its-owner
kind: doc
target_file: src/lib.rs
line_hint: 623
author: 
created: 
title: "drawn_by_its_owner"
tags: []
signature: "fn drawn_by_its_owner (doc : & Document , id : & str) -> bool"
---

Checks if a child control is drawn directly by its owning parent component.

An item its owner draws (#458: a `StatusPanel` inside its `StatusBar`)
has no place of its own on the canvas.
