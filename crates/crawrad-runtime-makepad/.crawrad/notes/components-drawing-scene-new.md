---
id: components-drawing-scene-new
kind: doc
target_file: src/components/drawing.rs
line_hint: 45
author: 
created: 
title: "Scene::new"
tags: []
signature: "pub fn new (todos : & 'a BTreeMap < NodeId , MakepadCtl >) -> Self"
---

Assembles the frame's scene out of the runtime's control map.

# Examples

```rust
use std::collections::BTreeMap;
use crawrad_runtime_makepad::components::Scene;

let map = BTreeMap::new();
let _scene = Scene::new(&map);
```
