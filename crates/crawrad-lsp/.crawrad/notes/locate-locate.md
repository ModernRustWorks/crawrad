---
id: locate-locate
kind: doc
target_file: src/locate.rs
line_hint: 89
author: 
created: 
title: "locate"
tags: []
signature: "pub fn locate () -> Result < LocatedServer , ServerMissing >"
---

The server this IDE uses, from the real environment.

# Examples

```rust,no_run
// Environment-dependent: requires rust-analyzer binary in PATH or IDE directory.
use crawrad_lsp::locate::locate;

let server = locate().unwrap();
assert!(server.path.is_file());
```

# Errors

Returns [`ServerMissing`] if no server binary could be found across all search locations.
