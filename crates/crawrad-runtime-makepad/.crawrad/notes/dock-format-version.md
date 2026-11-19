---
id: dock-format-version
kind: doc
target_file: src/dock.rs
line_hint: 15
author: 
created: 
title: "FORMAT_VERSION"
tags: []
signature: "pub const FORMAT_VERSION : u32 = 1"
---

The format version, written as the first line of the file.

It is a number of the format, not of the IDE: it goes up when the
meaning of what is written changes, and nothing else makes it move.

# Examples

```rust
use crawrad_runtime_makepad::dock::FORMAT_VERSION;

assert_eq!(FORMAT_VERSION, 1);
```
