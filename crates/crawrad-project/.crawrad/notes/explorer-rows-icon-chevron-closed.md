---
id: explorer-rows-icon-chevron-closed
kind: doc
target_file: src/explorer_rows/icon.rs
line_hint: 44
author: 
created: 
title: "CHEVRON_CLOSED"
tags: []
signature: "pub const CHEVRON_CLOSED : RowIcon = crate :: codicons :: CHEVRON_CLOSED"
---

The chevron of a node that is CLOSED — it points at what opening it shows.

Codicons `chevron-right`, and no longer the hand-written SDF triangle that
used to live in `CrawradFoldMark`. The triangle worked; it was a drawing of a
family of one, next to a column of Codicons, and the rule of the house is
that a loose glyph is not a set. It is also the shape the reference draws.

# Examples

```rust
use crawrad_project::explorer_rows::icon::CHEVRON_CLOSED;

assert!(!CHEVRON_CLOSED.is_empty());
```
