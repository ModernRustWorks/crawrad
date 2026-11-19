---
id: inspector-model-editors-collection-collection-window-text
kind: doc
target_file: src/inspector_model/editors/collection.rs
line_hint: 155
author: 
created: 
title: "Collection::window_text"
tags: []
signature: "fn window_text (& self) -> Option < String >"
---

The window text for editing the collection.

The text remains here, and **it is not leftover**: it is what
[`super::lines`] uses, what the egui shell uses when its collection
editor needs text, and what the round-trip tests measure. What changed
was which window the Makepad shell opens.
