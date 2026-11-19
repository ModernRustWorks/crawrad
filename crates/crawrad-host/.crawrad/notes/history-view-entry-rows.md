---
id: history-view-entry-rows
kind: doc
target_file: src/history_view.rs
line_hint: 242
author: 
created: 
title: "entry_rows"
tags: []
signature: "pub fn entry_rows (entries : & [(Entry , String)] , now : u64 , offset : i64) -> Vec < EntryRow >"
---

Formats history entries into table rows, newest first.

`now` and `offset` (seconds east of UTC) are parameters so tests are deterministic.

# Examples

```rust
use crawrad_host::history_view::entry_rows;

let rows = entry_rows(&[], 0, 0);
assert!(rows.is_empty());
```
