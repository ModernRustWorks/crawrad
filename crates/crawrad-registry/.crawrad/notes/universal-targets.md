---
id: universal-targets
kind: doc
target_file: src/lib.rs
line_hint: 603
author: 
created: 
title: "UNIVERSAL_TARGETS"
tags: []
signature: "pub const UNIVERSAL_TARGETS : & [& str] = & [UNIVERSAL_TARGET]"
---

Default collection of universal targets.

# Examples

```rust
use crawrad_registry::{UNIVERSAL_TARGET, UNIVERSAL_TARGETS};

assert!(UNIVERSAL_TARGETS.contains(&UNIVERSAL_TARGET));
```
