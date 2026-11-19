---
id: components-spin-edit-spin-edit
kind: doc
target_file: src/components/spin_edit.rs
line_hint: 4
author: 
created: 
title: "spin_edit"
tags: []
signature: "pub fn spin_edit () -> ComponentDesc"
---

A numeric input field with spin up and down buttons.

`SpinEdit` (#500, wave 1): a whole number typed or stepped with the up and
down buttons beside it. `Increment` is the step, `MinValue`/`MaxValue`
the range (both 0: no range, as in the classic RAD IDE), and
`EditorEnabled` off leaves only the buttons to change it.

# Examples

```rust
use crawrad_registry::spin_edit;

let desc = spin_edit();
assert_eq!(desc.class, "SpinEdit");
```
