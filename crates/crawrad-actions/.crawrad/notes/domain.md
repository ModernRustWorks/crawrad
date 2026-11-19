---
id: domain
kind: doc
target_file: src/lib.rs
line_hint: 398
author: 
created: 
title: "Domain"
tags: []
signature: "pub struct Domain"
---

The closed set of values a parameter accepts.

# Why it is a FUNCTION and not a slice

Because the values have to be **derived**, never written down again. The
domain of `panel` is [`Panel::ALL`]; the day a seventh panel lands, the
signature the MCP publishes and the rows the `View` menu draws both grow
with it and no list here is edited. A `&'static [Choice]` cannot be built
from `Panel::ALL` in a `const`, and writing the six out by hand would be
the second list this crate exists to avoid.

# Examples

```rust
use crawrad_actions::{Choice, Domain};

fn sample_choices() -> Vec<Choice> {
    vec![Choice { id: "a", label: "A" }]
}

let domain = Domain { values: sample_choices, row_per_value: false };
assert_eq!((domain.values)().len(), 1);
```
