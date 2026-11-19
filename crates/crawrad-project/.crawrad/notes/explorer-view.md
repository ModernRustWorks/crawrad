---
id: explorer-view
kind: doc
target_file: src/explorer.rs
line_hint: 249
author: 
created: 
title: "View"
tags: []
signature: "pub struct View < 'a >"
---

Everything the panel needs to know, gathered by the shell in one place.

A struct and not six arguments because the gate that tests the rows has to
build it, and six positional paths is how a test ends up passing the
document where the root goes.

# Examples

```rust
use std::collections::BTreeSet;
use crawrad_project::explorer::View;
use crawrad_project::explorer_rows::problems::ProblemMap;

let folds = BTreeSet::new();
let problems = ProblemMap::default();
let unsaved = BTreeSet::new();
let view = View {
    root: None,
    document: None,
    plan: &None,
    folds: &folds,
    problems: &problems,
    unsaved: &unsaved,
    legend_open: false,
    open_editors_open: false,
    outline_open: false,
    timeline_open: false,
    dependencies_open: false,
    timeline: None,
};
assert!(!view.legend_open);
```
