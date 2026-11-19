---
id: components-comum-entre
kind: doc
target_file: src/components/comum.rs
line_hint: 157
author: 
created: 
title: "entre"
tags: []
signature: "pub fn entre (v : f64 , piso : f64 , teto : f64) -> f64"
---

A length clamped between a floor and a ceiling **where the ceiling rules**.

It is the `f32::clamp` that does not panic, and is a deliberate copy of `entre` from RXP
(`crawrad-runtime-egui/src/lib.rs:2908`), for the same reason: Rust's `clamp`
**panics** when `min > max`, and that is exactly the shape of every
drawing floor when the rectangle shrinks — *"the thumb is at least 8
points, and never larger than the track"* holds true until the track measures 7.

The order is `max` **and then** `min`: **the floor is a wish, the ceiling is a
fact**. When the two cannot fit together, the fact wins.

# Examples

```rust
use crawrad_runtime_makepad::components::comum::entre;

assert_eq!(entre(5.0, 10.0, 20.0), 10.0);
assert_eq!(entre(25.0, 10.0, 20.0), 20.0);
```
