---
id: icons-chevron-closed
kind: doc
target_file: src/icons.rs
line_hint: 1656
author: 
created: 
title: "CHEVRON_CLOSED"
tags: []
signature: "pub const CHEVRON_CLOSED : & str = crate :: codicons :: CHEVRON_CLOSED"
---

The FOLD MARK — the pair indicating whether what is underneath is shown.

# Why it is not a [`Chrome`] nor lives in Explorer

It is not `Chrome` because `Chrome` is **button role** — what that button
does —, and the chevron does nothing by itself: it REPORTS a state, and the same
drawing serves anything that folds. Putting it in the `enum` would force two
roles (`FoldOpen`, `FoldClosed`) for a single thing, and `Chrome` would have to
decide which of the two via a state `if` inside a table that exists
precisely to avoid `if`s.

And it does not live in Explorer because **folding is not Explorer's**. The pair was
born there and stayed there while Explorer was the only panel that folded; when
Object Inspector gained collapsing categories, the choice was either importing a
constant from one panel into another — making Inspector depend on Explorer to
draw an arrow — or bringing the drawing to where drawings belong. With Explorer
rows outside the shell, that place is `crawrad_project::codicons`, which belongs to
no panel: **one definition, two readers**.

Codicons `chevron-right` and `chevron-down`, which is what the reference draws.

# Examples

```rust
use crawrad_project::icons::CHEVRON_CLOSED;

assert!(!CHEVRON_CLOSED.is_empty());
```
