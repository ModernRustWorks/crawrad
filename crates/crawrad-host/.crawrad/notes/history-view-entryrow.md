---
id: history-view-entryrow
kind: doc
target_file: src/history_view.rs
line_hint: 198
author: 
created: 
title: "EntryRow"
tags: []
signature: "pub struct EntryRow"
---

One line of the copies list / Information table.

# Examples

```rust
use crawrad_host::history_view::EntryRow;

let row = EntryRow {
    time: "12:00:00".into(),
    when: "today".into(),
    origin: "Save",
    size: "10 B".into(),
    lines: 1,
};
assert_eq!(row.origin, "Save");
```
