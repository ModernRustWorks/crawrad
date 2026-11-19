---
id: disk-watch-diskwatch-set-history-settings
kind: doc
target_file: src/disk_watch.rs
line_hint: 60
author: 
created: 
title: "DiskWatch::set_history_settings"
tags: []
signature: "pub fn set_history_settings (& mut self , settings : HistorySettings)"
---

Sets the local history settings.

# Examples

```rust
use crawrad_host::disk_watch::DiskWatch;
use crawrad_host::history::HistorySettings;

let mut dw = DiskWatch::default();
dw.set_history_settings(HistorySettings::default());
```
