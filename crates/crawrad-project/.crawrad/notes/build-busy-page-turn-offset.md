---
id: build-busy-page-turn-offset
kind: doc
target_file: src/build_busy.rs
line_hint: 98
author: 
created: 
title: "page_turn_offset"
tags: []
signature: "pub fn page_turn_offset (since_switch : Duration) -> f64"
---

Computes horizontal offset in pixels of the incoming page during a page turn.

Page turn: horizontal offset in px of the incoming page, `since_switch`
after a page switch: from PAGE_TURN_SHIFT (24.0) to 0.0 over
PAGE_TURN (150 ms), ease-out; 0.0 afterwards.

# Examples

```rust
use std::time::Duration;
use crawrad_project::build_busy::{page_turn_offset, PAGE_TURN_SHIFT};

assert_eq!(page_turn_offset(Duration::ZERO), PAGE_TURN_SHIFT);
assert_eq!(page_turn_offset(Duration::from_millis(200)), 0.0);
```
