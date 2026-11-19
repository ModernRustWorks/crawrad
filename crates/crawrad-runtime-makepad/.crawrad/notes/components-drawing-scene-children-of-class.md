---
id: components-drawing-scene-children-of-class
kind: doc
target_file: src/components/drawing.rs
line_hint: 67
author: 
created: 
title: "Scene::children_of_class"
tags: []
signature: "pub fn children_of_class < 'b > (& 'b self , c : & 'b MakepadCtl , class : & 'b str ,) -> impl Iterator < Item = & 'a MakepadCtl > + 'b"
---

The direct children of `c` of a given class.

# Examples

```rust,no_run
// MakepadCtl requires the Makepad runtime.
use std::collections::BTreeMap;
use crawrad_runtime_makepad::MakepadCtl;
use crawrad_runtime_makepad::components::Scene;

fn example(ctl: &MakepadCtl) {
    let map = BTreeMap::new();
    let scene = Scene::new(&map);
    assert_eq!(scene.children_of_class(ctl, "Button").count(), 0);
}
```
