---
id: components-status-bar-status-bar
kind: doc
target_file: src/components/status_bar.rs
line_hint: 7
author: 
created: 
title: "status_bar"
tags: []
signature: "pub fn status_bar () -> ComponentDesc"
---

Status bar window footer supporting informative panels.

`StatusBar`: status bar at the bottom edge of the window with information panels.

With `SimplePanel` on it shows `SimpleText`; off, it shows its `Panels`
(#458): `StatusPanel` children, side by side, each with its own width,
text and alignment — the classic RAD IDE's `Panels` collection, added
with the Structure's `+` like the actions of an `ActionList`.

# Examples

```rust
use crawrad_registry::status_bar;

let desc = status_bar();
assert_eq!(desc.class, "StatusBar");
```
