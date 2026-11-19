---
id: history-panel-historymode-as-str
kind: doc
target_file: src/history_panel.rs
line_hint: 323
author: 
created: 
title: "HistoryMode::as_str"
tags: []
signature: "pub fn as_str (& self) -> & 'static str"
---

Stable string identifier for MCP reporting.

# Examples

```rust
use crawrad_runtime_makepad::history_panel::HistoryMode;

assert_eq!(HistoryMode::Differences.as_str(), "differences");
```
