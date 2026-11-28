---
id: props-props-slint-border-radius
kind: doc
target_file: src/props.rs
line_hint: 2104
author: 
created: 
title: "Props::slint_border_radius"
tags: []
signature: "pub fn slint_border_radius (default : i64) -> PropDesc"
---

Slint's own corner rounding, offered only when Slint is the active target.

The `slint.` prefix is not decoration: `PropKey::parse` reads a dotted key as
belonging to that one target, so `props_for` hands this row to the Object
Inspector when Slint is generating and withholds it from Makepad and egui, which
express rounding differently or not at all. Before this there was exactly ONE
per-target property in the whole registry (`makepad.look`), so the mechanism
existed and had almost nothing to carry.

The default is whatever the Slint target emits for that component today, so
registering the property changes nothing until somebody turns it.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::slint_border_radius(4);
// The key carries the target, which is what filters it in the Inspector.
assert_eq!(p.key, "slint.border-radius");
```
