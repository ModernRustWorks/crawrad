---
id: history-view-diffrow
kind: doc
target_file: src/history_view.rs
line_hint: 26
author: 
created: 
title: "DiffRow"
tags: []
signature: "pub struct DiffRow"
---

One row of the side-by-side view; `None` on a side = an empty filler cell.

# Examples

```rust
use crawrad_host::history_view::{DiffRow, RowKind};

let row = DiffRow { kind: RowKind::Same, left: Some((1, "a".into())), right: Some((1, "a".into())) };
assert_eq!(row.kind, RowKind::Same);
```
