---
id: canvas-atlas-atlasentry
kind: doc
target_file: src/canvas/atlas.rs
line_hint: 22
author: 
created: 
title: "AtlasEntry"
tags: []
signature: "pub struct AtlasEntry"
---

Cached location and original dimensions of a picture in the atlas.

# Examples

```rust
use crawrad_runtime_makepad::canvas::atlas::AtlasEntry;

let entry = AtlasEntry {
    uv: [0.0, 0.0, 1.0, 1.0],
    original_width: 100,
    original_height: 50,
};
assert_eq!(entry.original_width, 100);
```
