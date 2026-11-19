---
id: targets-makepad-state-field
kind: doc
target_file: src/targets/makepad/state.rs
line_hint: 37
author: 
created: 
title: "Field"
tags: []
signature: "pub struct Field"
---

How this target stores a property outside the convention.

How this target stores a catalog property — when it does NOT store it
by convention.

# Examples

```rust
use crawrad_codegen::targets::makepad::state::Field;

let f = Field { name: "interval_ms", ty: "u64" };
assert_eq!(f.name, "interval_ms");
```
