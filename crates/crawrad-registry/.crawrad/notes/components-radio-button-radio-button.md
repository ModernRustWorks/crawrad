---
id: components-radio-button-radio-button
kind: doc
target_file: src/components/radio_button.rs
line_hint: 4
author: 
created: 
title: "radio_button"
tags: []
signature: "pub fn radio_button () -> ComponentDesc"
---

An exclusive radio button option control.

`RadioButton`: the exclusive choice. In the classic RAD IDE exclusion is between siblings of
the same parent — without containers yet, the parent is the form, which is what both
adapters do.

# Examples

```rust
use crawrad_registry::radio_button;

let desc = radio_button();
assert_eq!(desc.class, "RadioButton");
```
