---
id: options-opt-grade-min
kind: doc
target_file: src/options.rs
line_hint: 18
author: 
created: 
title: "OPT_GRADE_MIN"
tags: []
signature: "pub const OPT_GRADE_MIN : f32 = 2.0"
---

The limits for designer grid size, from the classic RAD IDE.

# Examples

```rust
use crawrad_project::options::{OPT_GRADE_MAX, OPT_GRADE_MIN};

assert!(OPT_GRADE_MIN < OPT_GRADE_MAX);
```
