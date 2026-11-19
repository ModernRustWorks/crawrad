---
id: types-slice-element
kind: doc
target_file: src/types.rs
line_hint: 148
author: 
created: 
title: "slice_element"
tags: []
signature: "pub fn slice_element (name : & str) -> Option < & str >"
---

Extracts the element type from a slice reference type name.

`Some(element)` for a slice reference: `&[T]` / `&mut [T]` from DWARF,
`ref$<slice2$<T> >` / `ref_mut$<slice2$<T> >` from a PDB (measured on
30/09/2026: `actions: &Actions` in a handler came as `ref$ { data_ptr,
length }`). Both have the children `data_ptr` and `length`.

# Examples

```rust
use crawrad_dap_rust::types::slice_element;

assert_eq!(slice_element("&[u8]"), Some("u8"));
```
