---
id: monitor-at-2
kind: doc
target_file: src/lib.rs
line_hint: 158
author: 
created: 
title: "monitor_at"
tags: []
signature: "pub fn monitor_at (_point : (f64 , f64)) -> Option < MonitorArea >"
---

The monitor that contains `point`, in physical pixels.

The monitor that contains `point` — see the Windows arm for the contract.

Always `None` here, and the reason is written there: the only caller wants
this to work around a Windows-specific maximize, and the other platforms
do not have the defect.

# Examples

```rust
use crawrad_os::monitor_at;

assert!(monitor_at((0.0, 0.0)).is_none());
```
