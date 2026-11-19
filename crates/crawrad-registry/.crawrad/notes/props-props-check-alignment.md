---
id: props-props-check-alignment
kind: doc
target_file: src/props.rs
line_hint: 1225
author: 
created: 
title: "Props::check_alignment"
tags: []
signature: "pub fn check_alignment (default : & 'static str) -> PropDesc"
---

Which side of the box the caption sits on (`Alignment`, #445).

Which side of the box the caption sits on (`Alignment` of a check box,
#445): `taRightJustify` puts it right of the box, `taLeftJustify` left
of it, with the box at the control's right edge.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::check_alignment("taRightJustify");
assert_eq!(p.key, "alignment");
```
