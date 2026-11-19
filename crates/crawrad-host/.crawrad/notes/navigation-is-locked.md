---
id: navigation-is-locked
kind: doc
target_file: src/navigation.rs
line_hint: 294
author: 
created: 
title: "is_locked"
tags: []
signature: "pub fn is_locked (file : & Path , generated_suffix : & str) -> bool"
---

[`locked`] with this machine's [`foreign_roots`].

# Examples

```rust
use crawrad_host::navigation::is_locked;
use std::path::Path;

assert!(is_locked(Path::new("foo.designer.rs"), ".designer.rs"));
```
