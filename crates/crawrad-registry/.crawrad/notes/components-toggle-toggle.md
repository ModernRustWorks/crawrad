---
id: components-toggle-toggle
kind: doc
target_file: src/components/toggle.rs
line_hint: 4
author: 
created: 
title: "toggle"
tags: []
signature: "pub fn toggle () -> ComponentDesc"
---

An on/off toggle switch with a sliding knob.

`Toggle` (#500, wave 1): an on/off switch with its caption — the
`CheckBox`'s state drawn as a sliding knob. `Checked` is on.

# Examples

```rust
use crawrad_registry::toggle;

let desc = toggle();
assert_eq!(desc.class, "Toggle");
```
