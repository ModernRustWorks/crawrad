---
id: manifest-componententry
kind: doc
target_file: src/manifest.rs
line_hint: 555
author: 
created: 
title: "ComponentEntry"
tags: []
signature: "pub struct ComponentEntry"
---

A component, in the file format.

# Examples

```rust
use crawrad_package::manifest::{Children, ComponentEntry};

let comp = ComponentEntry {
    class: "Btn".into(),
    display: "Button".into(),
    palette: "Standard".into(),
    prefix: "btn".into(),
    base_name: None,
    default_size: [80.0, 24.0],
    visual: true,
    emits_code: true,
    private_state: false,
    children: Children::None,
    item_class: None,
    derived_geometry: false,
    text_line: None,
    targets: vec!["*".into()],
    provides: Vec::new(),
    props: Vec::new(),
    events: Vec::new(),
    draw: Vec::new(),
    icon: Vec::new(),
    requires: Vec::new(),
    inherits: None,
};
assert_eq!(comp.class, "Btn");
```
