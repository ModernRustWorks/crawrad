---
id: tabs-window
kind: doc
target_file: src/tabs.rs
line_hint: 55
author: 
created: 
title: "Window"
tags: []
signature: "pub struct Window"
---

Window of visible tabs displayed on screen in a tab strip.

The tabs of a strip that are on screen (#13): the window of the shown tabs
that holds the open one. `shown[i]` is whether tab `i` is visible
(`TabVisible`); the rule itself is
[`crate::shared_geometry::tab_window`], the copy of `crawrad_registry::tab_window`.

# Examples

```rust
use crawrad_runtime_makepad::tabs::window;

let w = window(500.0, &[true, true, true], 0);
assert!(w.contains(0));
```
