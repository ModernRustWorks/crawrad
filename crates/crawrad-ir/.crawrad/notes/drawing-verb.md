---
id: drawing-verb
kind: doc
target_file: src/drawing.rs
line_hint: 162
author: 
created: 
title: "Verb"
tags: []
signature: "pub struct Verb"
---

**A PAINT VERB**: where, and what.

The two questions are separated on purpose — see [`Area`], which was born
when a gauge did not fit in five verbs that only knew how to paint the entire
box.

# Examples

```rust
use crawrad_ir::drawing::{Area, Ink, Stroke, Verb};

let verb = Verb::all(Stroke::Dashed { color: Ink::Border });
assert_eq!(verb.area, Area::All);
```
