---
id: drawing-limit
kind: doc
target_file: src/drawing.rs
line_hint: 118
author: 
created: 
title: "Limit"
tags: []
signature: "pub enum Limit"
---

One end of the range of an [`Area::Fraction`].

**ONE END OF THE RANGE of an [`Area::Fraction`]** — fixed number, or the value of
a property.

# Examples

```rust
use crawrad_ir::drawing::Limit;

let limit = Limit::Fixed(100.0);
assert_eq!(limit.property(), None);
```

# It was born from a small lie, and that lie belongs to the expensive family

The first version accepted only numbers. `Gauge` declared
`min = 0.0, max = 100.0` in the drawing and its crate exposed fields `min` and
`max` — and the two did not talk to each other: changing `gauge.min` at runtime changed the
component's `fraction()` and **did not change the painted bar**.

As long as the values were the defaults, both agreed. It is exactly the
form of defect that this house hunts down: right until someone uses it, and then the
designer shows one thing and the application shows another.

# Why it did not become "every end is a property"

Because most are not. The scrollbar of a declared component, the
progress bar from 0 to 100, the percentage meter — all have a
FIXED range, and forcing them to declare two extra properties just to say `0` and
`100` would be the format charging a tax on those who don't need it.
