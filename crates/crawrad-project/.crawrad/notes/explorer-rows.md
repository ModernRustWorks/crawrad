---
id: explorer-rows
kind: doc
target_file: src/explorer.rs
line_hint: 275
author: 
created: 
title: "rows"
tags: []
signature: "pub fn rows (view : & View) -> Vec < Row >"
---

The whole panel, as a list of rows.

It is the same order the egui panel paints top to bottom, and every
decision inside it is delegated: [`model::group_tree`] for the
members, [`model::member_files`] for whether a member may show badges
at all, [`model::summary`] for the count line and [`model::legend`] for the
legend. Nothing here re-decides any of it.

# Examples

```rust
use std::collections::BTreeSet;
use crawrad_project::explorer::{rows, View};
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
let r = rows(&view);
assert_eq!(r.len(), 1);
```
