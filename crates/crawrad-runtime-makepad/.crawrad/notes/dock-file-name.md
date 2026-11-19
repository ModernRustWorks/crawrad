---
id: dock-file-name
kind: doc
target_file: src/dock.rs
line_hint: 12
author: 
created: 
title: "FILE_NAME"
tags: []
signature: "pub const FILE_NAME : & str = \"makepad-dock-layout.ron\""
---

The name of the arrangement file inside the user's configuration folder.

# Examples

```rust
use crawrad_runtime_makepad::dock::FILE_NAME;

assert!(FILE_NAME.ends_with(".ron"));
```
