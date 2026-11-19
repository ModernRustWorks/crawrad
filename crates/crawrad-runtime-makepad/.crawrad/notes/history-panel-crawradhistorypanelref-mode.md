---
id: history-panel-crawradhistorypanelref-mode
kind: doc
target_file: src/history_panel.rs
line_hint: 721
author: 
created: 
title: "CrawradHistoryPanelRef::mode"
tags: []
signature: "pub fn mode (& self) -> HistoryMode"
---

The mode on screen (Differences when the panel is not there).

# Examples

```rust
use crawrad_runtime_makepad::history_panel::{HistoryMode, CrawradHistoryPanelRef};

let r = CrawradHistoryPanelRef::default();
assert_eq!(r.mode(), HistoryMode::Differences);
```
