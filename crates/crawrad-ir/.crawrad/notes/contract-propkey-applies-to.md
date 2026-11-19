---
id: contract-propkey-applies-to
kind: doc
target_file: src/contract.rs
line_hint: 195
author: 
created: 
title: "PropKey::applies_to"
tags: []
signature: "pub fn applies_to (& self , target : & str) -> bool"
---

Returns whether this property applies to the specified runtime.

Does this property apply to the specified runtime?

# Examples

```rust
use crawrad_ir::PropKey;

let common = PropKey::parse("text").unwrap();
assert!(common.applies_to("egui"));
assert!(common.applies_to("makepad"));

let target = PropKey::parse("egui.desired_width").unwrap();
assert!(target.applies_to("egui"));
assert!(!target.applies_to("makepad"));
```
