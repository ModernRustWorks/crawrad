---
id: components-drawing-scene-find
kind: doc
target_file: src/components/drawing.rs
line_hint: 55
author: 
created: 
title: "Scene::find"
tags: []
signature: "pub fn find (& self , name_or_id : & str) -> Option < & 'a MakepadCtl >"
---

Looks up a control by id or by name.

# Examples

```rust
use std::collections::BTreeMap;
use crawrad_runtime_makepad::components::Scene;

let map = BTreeMap::new();
let scene = Scene::new(&map);
assert!(scene.find("btn1").is_none());
```
