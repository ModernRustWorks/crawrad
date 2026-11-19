---
id: ide-app-layout-left-group
kind: doc
target_file: src/ide_app/layout.rs
line_hint: 53
author: 
created: 
title: "LEFT_GROUP"
tags: []
signature: "pub const LEFT_GROUP : LiveId = live_id ! (dock_left)"
---

The tab group each panel lands in when the IDE ships.

They are also the three drop targets a `Move To` aims at: the leftmost
group is the left edge, the rightmost is the right edge, and the bottom is
created by dropping below the centre.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
assert_ne!(LEFT_GROUP, CENTER_GROUP);
```
