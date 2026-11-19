---
id: build-settings-already-building
kind: doc
target_file: src/build_settings.rs
line_hint: 166
author: 
created: 
title: "ALREADY_BUILDING"
tags: []
signature: "pub const ALREADY_BUILDING : & str = \"already building — wait for it to finish\""
---

What the bar says when a command is asked for while a build runs.

# Examples

```rust
use crawrad_project::build_settings::ALREADY_BUILDING;

assert!(ALREADY_BUILDING.contains("already building"));
```
