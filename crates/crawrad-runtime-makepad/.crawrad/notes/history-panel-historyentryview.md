---
id: history-panel-historyentryview
kind: doc
target_file: src/history_panel.rs
line_hint: 363
author: 
created: 
title: "HistoryEntryView"
tags: []
signature: "pub struct HistoryEntryView"
---

A formatted entry for display in the history panel.

# Examples

```rust
use crawrad_runtime_makepad::history_panel::HistoryEntryView;

let entry = HistoryEntryView {
    time: "10:00".into(),
    when: "today".into(),
    origin: "internal".into(),
    size: "1 KB".into(),
    lines: 42,
};
assert_eq!(entry.lines, 42);
```
