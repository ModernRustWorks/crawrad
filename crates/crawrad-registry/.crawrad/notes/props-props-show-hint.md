---
id: props-props-show-hint
kind: doc
target_file: src/props.rs
line_hint: 219
author: 
created: 
title: "Props::show_hint"
tags: []
signature: "pub fn show_hint (default : bool) -> PropDesc"
---

Whether the hint shows as a tooltip on hover (`ShowHint`).

Whether the `Hint` shows as a tooltip when the pointer rests on the
control (`ShowHint`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::show_hint(true);
assert_eq!(p.key, "show_hint");
```
