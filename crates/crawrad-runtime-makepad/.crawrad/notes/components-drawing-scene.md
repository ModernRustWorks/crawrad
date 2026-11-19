---
id: components-drawing-scene
kind: doc
target_file: src/components/drawing.rs
line_hint: 39
author: 
created: 
title: "Scene"
tags: []
signature: "pub struct Scene < 'a >"
---

What a component may READ of its own neighbourhood while it draws.

It is reading, and nothing else: the `Scene` has no way of changing the
model. It exists because the `PageControl` draws the caption of its pages,
which are sibling nodes in the document — and because the next step (events
per component) will need that same neighbourhood without any file gaining a
new channel to the runtime.

# Examples

```rust
use std::collections::BTreeMap;
use crawrad_runtime_makepad::components::Scene;

let map = BTreeMap::new();
let scene = Scene::new(&map);
assert!(scene.ctl("none").is_none());
```
