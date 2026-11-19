---
id: targets-splitters-split
kind: doc
target_file: src/targets/splitters.rs
line_hint: 13
author: 
created: 
title: "Split"
tags: []
signature: "pub struct Split"
---

One Splitter and the two controls its long edges touch.

# Examples

```rust
use crawrad_codegen::targets::splitters::Split;

let s = Split {
    field: "split1".to_string(),
    vertical: true,
    before: None,
    after: None,
};
assert_eq!(s.field, "split1");
```
