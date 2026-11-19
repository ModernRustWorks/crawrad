---
id: components-drawing-scene-children
kind: doc
target_file: src/components/drawing.rs
line_hint: 62
author: 
created: 
title: "Scene::children"
tags: []
signature: "pub fn children < 'b > (& 'b self , c : & 'b MakepadCtl) -> impl Iterator < Item = & 'a MakepadCtl > + 'b"
---

The direct children of `c`, in the order the document keeps them.

# Examples

```rust,no_run
// MakepadCtl requires the Makepad runtime.
use std::collections::BTreeMap;
use crawrad_runtime_makepad::MakepadCtl;
use crawrad_runtime_makepad::components::Scene;

fn example(ctl: &MakepadCtl) {
    let map = BTreeMap::new();
    let scene = Scene::new(&map);
    assert_eq!(scene.children(ctl).count(), 0);
}
```
