---
id: paint-command
kind: doc
target_file: src/paint.rs
line_hint: 59
author: 
created: 
title: "Command"
tags: []
signature: "pub enum Command"
---

A 2D draw primitive command in local coordinates.

One thing to draw, in the box's own coordinates; colours are bytes and
`None` leaves that part out.

# Examples

```rust
use crawrad_runtime_makepad::paint::Command;

let cmd = Command::Rect {
    x: 0.0, y: 0.0, w: 10.0, h: 10.0, radius: 0.0,
    fill: Some([255, 0, 0, 255]), stroke: None,
};
assert!(matches!(cmd, Command::Rect { .. }));
```
