---
id: components-track-bar-track-bar
kind: doc
target_file: src/components/track_bar.rs
line_hint: 4
author: 
created: 
title: "track_bar"
tags: []
signature: "pub fn track_bar () -> ComponentDesc"
---

`TrackBar`: slider control for visual selection of continuous values.

# Examples

```rust
use crawrad_registry::track_bar;

let desc = track_bar();
assert_eq!(desc.class, "TrackBar");
```
