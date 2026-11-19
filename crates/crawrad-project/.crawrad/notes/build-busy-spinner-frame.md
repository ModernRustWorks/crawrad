---
id: build-busy-spinner-frame
kind: doc
target_file: src/build_busy.rs
line_hint: 13
author: 
created: 
title: "spinner_frame"
tags: []
signature: "pub fn spinner_frame (since_start : Duration) -> & 'static str"
---

Frame of the spinner `since_start` after the run began, one step per 120 ms.

# Examples

```rust
use std::time::Duration;
use crawrad_project::build_busy::spinner_frame;

assert_eq!(spinner_frame(Duration::from_millis(0)), "◐");
assert_eq!(spinner_frame(Duration::from_millis(120)), "◓");
```
