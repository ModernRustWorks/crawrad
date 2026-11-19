---
id: history-panel-can-navigate-blocks
kind: doc
target_file: src/history_panel.rs
line_hint: 343
author: 
created: 
title: "can_navigate_blocks"
tags: []
signature: "pub fn can_navigate_blocks (blocks : usize) -> bool"
---

Returns whether change navigation buttons should be enabled.

# Examples

```rust
use crawrad_runtime_makepad::history_panel::can_navigate_blocks;

assert!(can_navigate_blocks(1));
assert!(!can_navigate_blocks(0));
```
