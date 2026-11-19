---
id: props-props-edit-mask
kind: doc
target_file: src/props.rs
line_hint: 1082
author: 
created: 
title: "Props::edit_mask"
tags: []
signature: "pub fn edit_mask (default : & str) -> PropDesc"
---

The formatting mask for a `MaskEdit` (`EditMask`, #71).

The mask a `MaskEdit` shapes its text by (`EditMask`, #71): the
classic syntax `crawrad_runtime::mask` reads. Empty: no mask, an `Edit`.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::edit_mask("!99/99/00;1;_");
assert_eq!(p.key, "edit_mask");
```
