---
id: build-settings-targetplatform-set-in
kind: doc
target_file: src/build_settings.rs
line_hint: 79
author: 
created: 
title: "TargetPlatform::set_in"
tags: []
signature: "pub fn set_in (selected : & mut Vec < Self > , index : usize , on : bool)"
---

Ticks or unticks the target at index within the selected target list.

Ticks (`on`) or unticks the target at `index` of `ALL` in `selected`,
without duplicating it; an index past `ALL` does nothing.

# Examples

```rust
use crawrad_project::build_settings::TargetPlatform;

let mut selected = Vec::new();
TargetPlatform::set_in(&mut selected, 0, true);
assert_eq!(selected.len(), 1);
```
