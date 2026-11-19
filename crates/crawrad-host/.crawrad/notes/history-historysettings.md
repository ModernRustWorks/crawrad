---
id: history-historysettings
kind: doc
target_file: src/history.rs
line_hint: 50
author: 
created: 
title: "HistorySettings"
tags: []
signature: "pub struct HistorySettings"
---

What Options sets for the local history.

# Examples

```rust
use crawrad_host::history::HistorySettings;

let settings = HistorySettings::default();
assert!(settings.enabled);
```
