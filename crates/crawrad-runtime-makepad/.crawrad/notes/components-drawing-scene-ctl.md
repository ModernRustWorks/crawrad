---
id: components-drawing-scene-ctl
kind: doc
target_file: src/components/drawing.rs
line_hint: 50
author: 
created: 
title: "Scene::ctl"
tags: []
signature: "pub fn ctl (& self , id : & str) -> Option < & 'a MakepadCtl >"
---

One control by id, if it exists in the document.

# Examples

```rust
use std::collections::BTreeMap;
use crawrad_runtime_makepad::components::Scene;

let map = BTreeMap::new();
let scene = Scene::new(&map);
assert!(scene.ctl("btn1").is_none());
```
