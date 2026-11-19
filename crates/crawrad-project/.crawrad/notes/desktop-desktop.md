---
id: desktop-desktop
kind: doc
target_file: src/desktop.rs
line_hint: 46
author: 
created: 
title: "Desktop"
tags: []
signature: "pub struct Desktop"
---

Everything the desktop holds.

# Examples

```rust
use crawrad_project::desktop::Desktop;

let d = Desktop::default();
assert!(d.tabs.is_empty());
```
