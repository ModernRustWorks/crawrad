---
id: snapghost
kind: doc
target_file: src/lib.rs
line_hint: 173
author: 
created: 
title: "SnapGhost"
tags: []
signature: "pub struct SnapGhost"
---

Describes the offset of a dragged anchor relative to where it snaps.

Where the pointer would have put the dragged anchor, relative to where it
snaps (#564): the canvas draws the landing and this offset so Snap shows.

# Examples

```rust
use crawrad_designer::SnapGhost;
use crawrad_ir::NodeId;

let ghost = SnapGhost { node: NodeId::from("btn"), dx: 2.0, dy: 3.0 };
assert_eq!(ghost.dx, 2.0);
```
