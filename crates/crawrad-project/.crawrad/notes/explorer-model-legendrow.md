---
id: explorer-model-legendrow
kind: doc
target_file: src/explorer_model.rs
line_hint: 317
author: 
created: 
title: "LegendRow"
tags: []
signature: "pub struct LegendRow"
---

One row of the legend: the word, and what it means.

It is a type of the MODEL and not of a shell because the pair is the same on
both screens — the egui panel paints it in a `horizontal`, the Makepad one as
a list row, and neither of them gets to pick different words.

# Examples

```rust
use crawrad_project::explorer_model::LegendRow;

let row = LegendRow {
    mark: "active",
    phrase: "active member",
    warn: false,
};
assert_eq!(row.mark, "active");
```
