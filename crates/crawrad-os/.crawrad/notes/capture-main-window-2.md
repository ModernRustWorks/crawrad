---
id: capture-main-window-2
kind: doc
target_file: src/lib.rs
line_hint: 511
author: 
created: 
title: "capture_main_window"
tags: []
signature: "pub fn capture_main_window (_region : Option < [i32 ; 4] >) -> Option < Capture >"
---

Captures nothing: the capture is Windows-only for now.

# Examples

```
assert!(crawrad_os::capture_main_window(None).is_none());
```
