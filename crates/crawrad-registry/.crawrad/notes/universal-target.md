---
id: universal-target
kind: doc
target_file: src/lib.rs
line_hint: 600
author: 
created: 
title: "UNIVERSAL_TARGET"
tags: []
signature: "pub const UNIVERSAL_TARGET : & str = \"*\""
---

Wildcard indicating that a component is universal (supported by any runtime).

# Examples

```rust
use crawrad_registry::UNIVERSAL_TARGET;

assert_eq!(UNIVERSAL_TARGET, "*");
```
