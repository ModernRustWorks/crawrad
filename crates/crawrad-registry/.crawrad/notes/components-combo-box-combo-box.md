---
id: components-combo-box-combo-box
kind: doc
target_file: src/components/combo_box.rs
line_hint: 4
author: 
created: 
title: "combo_box"
tags: []
signature: "pub fn combo_box () -> ComponentDesc"
---

A drop-down selection list.

`ComboBox`: drop-down list. Items are the same collection as `TreeView`
— `crawrad_ir::Item` — used flat, without children. This is what §14 foresaw:
solving the collection once solves combo, list, and grid columns.

Only the **list** style (classic RAD IDE's `csDropDownList`): chosen by
`ItemIndex`, and there is no typeable text. The editable style is effectively
another component — on Win32 it is a different combination of styles, in egui it is a
field with a menu next to it —, and pretending it is the same would cost exactly the
divergence between targets that ADR 0002 exists to prevent.

# Examples

```rust
use crawrad_registry::combo_box;

let desc = combo_box();
assert_eq!(desc.class, "ComboBox");
```
