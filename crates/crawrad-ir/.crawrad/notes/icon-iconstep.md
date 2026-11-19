---
id: icon-iconstep
kind: doc
target_file: src/icon.rs
line_hint: 11
author: 
created: 
title: "IconStep"
tags: []
signature: "pub enum IconStep"
---

One step of an icon drawing, in grid units.

One step of an icon drawing, in grid units. Shapes accumulate until a
`Fill` or a `Stroke` paints them with the theme ink, as in `Sdf2d`.

# Examples

```rust
use crawrad_ir::icon::IconStep;

let step = IconStep::MoveTo(0.0, 0.0);
assert!(matches!(step, IconStep::MoveTo(..)));
```
