---
id: ide-app-layout-dock
kind: doc
target_file: src/ide_app/layout.rs
line_hint: 50
author: 
created: 
title: "DOCK"
tags: []
signature: "pub const DOCK : & [LiveId] = id ! (dock)"
---

The `Dock` widget of the shell, addressed from Rust.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
assert_eq!(DOCK.len(), 1);
```
