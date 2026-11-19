---
id: dwarf-arm
kind: doc
target_file: src/dwarf.rs
line_hint: 39
author: 
created: 
title: "Arm"
tags: []
signature: "pub struct Arm"
---

One arm of a Rust enum's variant part.

# Examples

```rust
use crawrad_dap_rust::dwarf::{Arm, Member};

let arm = Arm {
    discr_value: Some(0),
    member: Member {
        name: "None".into(),
        offset: 0,
        ty: None,
    },
};
assert_eq!(arm.discr_value, Some(0));
```
