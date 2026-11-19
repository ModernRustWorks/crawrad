---
id: handles
kind: doc
target_file: src/lib.rs
line_hint: 309
author: 
created: 
title: "handles"
tags: []
signature: "pub fn handles (x : f32 , y : f32 , w : f32 , h : f32) -> [(f32 , f32) ; 8]"
---

Calculates the eight handle points clockwise from top-left.

The eight points where a rectangle's handles sit, **clockwise starting
from the top-left corner**.

A single function, and that is the point: what **renders** the handle and what **looks** for clicks
on it need the exact same numbers. This was previously written twice — `cantos`
in the adapter and `Handle::anchor` in the designer — and the two agreed by chance,
with nothing enforcing it. Moving a handle on one side would place rendering in one spot and
clicking in another, and the symptom would be "the handle does not catch", which nobody links
to that cause.

The order is a contract: `Handle::ALL` and the adapter's cursor table index
by it.

# Examples

```rust
use crawrad_runtime::handles;

let pts = handles(10.0, 10.0, 100.0, 50.0);
assert_eq!(pts.len(), 8);
```
