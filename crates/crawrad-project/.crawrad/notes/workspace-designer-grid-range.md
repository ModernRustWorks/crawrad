---
id: workspace-designer-grid-range
kind: doc
target_file: src/workspace.rs
line_hint: 1128
author: 
created: 
title: "DESIGNER_GRID_RANGE"
tags: []
signature: "pub const DESIGNER_GRID_RANGE : std :: ops :: RangeInclusive < u32 > = 2 ..= 64"
---

The grid sizes the Options accept, as the classic RAD IDE.

The grid sizes the Options accept, as the classic RAD IDE: below 2 there
are no grid points to draw.

# Examples

```rust
use crawrad_project::workspace::DESIGNER_GRID_RANGE;

assert!(DESIGNER_GRID_RANGE.contains(&8));
```
