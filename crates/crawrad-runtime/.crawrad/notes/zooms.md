---
id: zooms
kind: doc
target_file: src/lib.rs
line_hint: 331
author: 
created: 
title: "ZOOMS"
tags: []
signature: "pub const ZOOMS : [f32 ; 8] = [0.25 , 0.5 , 0.75 , 1.0 , 1.5 , 2.0 , 3.0 , 4.0]"
---

The steps of canvas zoom, in ascending order.

**The classic RAD IDE has no designer zoom**, so here the visual contract does not
rule and the ladder came from where it has existed for decades: drawing
tools. Photoshop, Illustrator, and Figma use the same fractions — half,
double, quadruple — and a hand coming from there feels right at home.

Discrete, rather than continuous, for two reasons:

- **`1.0` must be reachable again.** On a continuous scale, a user
  goes past 100% and cannot find their way back — and 100% is the only zoom where
  what they see is what the application will show. Here it is a step, and the
  ladder always stops at it.
- **Round fractions do not accumulate error.** A multiplicative ladder (`× 1.1` on
  each step) leaves `1.0` and never returns, and the "100%" on the bar begins to lie
  due to rounding.

# Examples

```rust
use crawrad_runtime::ZOOMS;

assert_eq!(ZOOMS[0], 0.25);
assert_eq!(ZOOMS[3], 1.0);
```
