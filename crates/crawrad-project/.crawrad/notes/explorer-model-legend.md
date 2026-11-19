---
id: explorer-model-legend
kind: doc
target_file: src/explorer_model.rs
line_hint: 329
author: 
created: 
title: "legend"
tags: []
signature: "pub fn legend (lines : & [Line] , members : & [MemberNode]) -> Vec < LegendRow >"
---

The panel legend — **only what is on screen**.

Explaining badges that do not appear wastes lines on questions no one asked, and
the legend lives at the bottom of a narrow panel, where each line competes with
the file list. Therefore each pair enters only if there is a row (or member) that
displays it.

It lives in the model because BOTH shells render it. Written in both, the second
would be born with yesterday's phrase — and an outdated legend is the most silent
way for a legend to lie.

# Examples

```rust
use crawrad_project::explorer_model::legend;

let rows = legend(&[], &[]);
assert!(rows.is_empty());
```
