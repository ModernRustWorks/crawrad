---
id: monitorarea
kind: doc
target_file: src/lib.rs
line_hint: 97
author: 
created: 
title: "MonitorArea"
tags: []
signature: "pub struct MonitorArea"
---

One monitor, told twice: everything it has and the part a window may use.

Both come back because the DIFFERENCE is the interesting part — it is the
taskbar, and a caller that wants to know whether there is one at all
compares the two. Handing back only [`MonitorArea::work`] would make that
unanswerable without a second call.

# Examples

```rust
use crawrad_os::{MonitorArea, ScreenRect};

let area = MonitorArea::default();
assert_eq!(area.full, ScreenRect::default());
```
