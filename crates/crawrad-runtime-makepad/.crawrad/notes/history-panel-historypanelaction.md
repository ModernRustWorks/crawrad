---
id: history-panel-historypanelaction
kind: doc
target_file: src/history_panel.rs
line_hint: 373
author: 
created: 
title: "HistoryPanelAction"
tags: []
signature: "pub enum HistoryPanelAction"
---

Actions emitted by user clicks in the history panel.

# Examples

```rust
use crawrad_runtime_makepad::history_panel::HistoryPanelAction;

let action = HistoryPanelAction::Previous;
assert_eq!(action, HistoryPanelAction::Previous);
```
