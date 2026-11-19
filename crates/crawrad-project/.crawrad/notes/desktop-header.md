---
id: desktop-header
kind: doc
target_file: src/desktop.rs
line_hint: 12
author: 
created: 
title: "HEADER"
tags: []
signature: "pub const HEADER : & str = \"crawrad-desktop 1\""
---

The first line; anything else is not a desktop this build can read.

# Examples

```rust
use crawrad_project::desktop::HEADER;

assert_eq!(HEADER, "crawrad-desktop 1");
```
