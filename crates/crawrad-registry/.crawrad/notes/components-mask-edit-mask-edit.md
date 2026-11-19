---
id: components-mask-edit-mask-edit
kind: doc
target_file: src/components/mask_edit.rs
line_hint: 4
author: 
created: 
title: "mask_edit"
tags: []
signature: "pub fn mask_edit () -> ComponentDesc"
---

An edit box whose input text is formatted by a mask.

`MaskEdit` (#71): an `Edit` whose text is shaped by its `EditMask` — the
same properties and events, plus the mask.

Its `Text` is not offered for data binding: the generators bind only the
`Edit`'s, and an offer they would drop is a lie in the Object Inspector.

# Examples

```rust
use crawrad_registry::mask_edit;

let desc = mask_edit();
assert_eq!(desc.class, "MaskEdit");
```
