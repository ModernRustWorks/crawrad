---
id: history-view-historysession
kind: doc
target_file: src/history_view.rs
line_hint: 298
author: 
created: 
title: "HistorySession"
tags: []
signature: "pub struct HistorySession"
---

State and operations for the local history view of a file.

The History view of one file: its copies, the one compared with the
buffer and the change block in focus. Rows and `selected` count newest
first, as the list shows them.

# Examples

```rust
use crawrad_host::history_view::HistorySession;

let session = HistorySession::default();
assert_eq!(session.copy(), "");
```
