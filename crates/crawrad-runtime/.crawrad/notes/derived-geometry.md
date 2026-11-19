---
id: derived-geometry
kind: doc
target_file: src/lib.rs
line_hint: 253
author: 
created: 
title: "DERIVED_GEOMETRY"
tags: []
signature: "pub const DERIVED_GEOMETRY : & [& str] = & [\"TabSheet\"]"
---

Classes whose rectangle geometry is imposed by the parent.

The classes whose rectangle the parent imposes (a `TabSheet` fills its
`PageControl`): they get no resize handles nor resize cursor (#113). It
mirrors the catalogue's `derived_geometry`; the runtime-makepad test
`derived_geometry_matches_the_catalogue` keeps the two equal.

# Examples

```rust
use crawrad_runtime::DERIVED_GEOMETRY;

assert!(DERIVED_GEOMETRY.contains(&"TabSheet"));
```
