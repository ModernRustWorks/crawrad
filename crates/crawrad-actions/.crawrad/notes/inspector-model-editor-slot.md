---
id: inspector-model-editor-slot
kind: doc
target_file: src/inspector_model/editor.rs
line_hint: 38
author: 
created: 
title: "Slot"
tags: []
signature: "pub enum Slot"
---

A named widget inside the template that draws a row.

# Why a closed set, and not the name of the widget as a string

Because the shell has to turn it into an `id!(…)` path, and `id!` takes a
literal. A `&'static str` would mean `LiveId::from_str` at runtime and a
typo that resolves to nothing — the empty `WidgetRef` that draws nothing
and says nothing, which is the whole family of defect the DSL gate exists
to catch. With a closed set, the translation is one exhaustive `match` in
the shell and a slot nobody translated does not compile.

Four of them are the quarters of a `Rect`, and they are deliberately named
after position rather than after meaning: they are `x y w h` in a `Rect`
and they would be `left top right bottom` in anything else. A slot is a
place in a template, not a concept.

# Examples

```rust
use crawrad_actions::inspector_model::editor::Slot;

let slot = Slot::Value;
assert_eq!(slot, Slot::Value);
```
