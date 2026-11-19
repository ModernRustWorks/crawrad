---
id: components-check-box-check-box
kind: doc
target_file: src/components/check_box.rs
line_hint: 4
author: 
created: 
title: "check_box"
tags: []
signature: "pub fn check_box () -> ComponentDesc"
---

A check box control that toggles between checked and unchecked states.

`CheckBox`: checkbox. `Checked` belongs to the common core — both
adapters honor it, and it is what the generated application reads and writes.

# Examples

```rust
use crawrad_registry::check_box;

let desc = check_box();
assert_eq!(desc.class, "CheckBox");
```
