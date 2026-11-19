---
id: props-badge-kinds
kind: doc
target_file: src/props.rs
line_hint: 82
author: 
created: 
title: "BADGE_KINDS"
tags: []
signature: "pub const BADGE_KINDS : & [& str] = & [\"bkDefault\" , \"bkPrimary\" , \"bkInfo\" , \"bkWarning\" , \"bkDanger\"]"
---

The kind styles of a `Badge` (#500).

# Examples

```rust
use crawrad_registry::BADGE_KINDS;

assert!(BADGE_KINDS.contains(&"bkDefault"));
```
