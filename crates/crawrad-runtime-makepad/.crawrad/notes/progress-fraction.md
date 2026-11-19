---
id: progress-fraction
kind: doc
target_file: src/progress.rs
line_hint: 12
author: 
created: 
title: "fraction"
tags: []
signature: "pub fn fraction (position : i64 , min : i64 , max : i64) -> f64"
---

Computes progress fraction clamped to `0.0..=1.0` between `min` and `max`.

How full a ProgressBar is: `position` between `min` and `max`, clamped to
`0..=1`. An empty or inverted range shows empty.

# Examples

```rust
use crawrad_runtime_makepad::progress::fraction;

assert_eq!(fraction(50, 0, 100), 0.5);
assert_eq!(fraction(150, 0, 100), 1.0);
assert_eq!(fraction(10, 100, 50), 0.0);
```
