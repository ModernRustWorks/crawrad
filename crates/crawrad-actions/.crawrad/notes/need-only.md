---
id: need-only
kind: doc
target_file: src/lib.rs
line_hint: 1
author: 
created: 
title: "Need::only"
tags: []
signature: "pub fn only (self) -> Gate"
---

The gate of an action whose need alone decides.

# Examples

```rust
use crawrad_actions::Need;

assert_eq!(Need::Project.only().need, Need::Project);
```
