---
id: history-view-previous-block
kind: doc
target_file: src/history_view.rs
line_hint: 184
author: 
created: 
title: "previous_block"
tags: []
signature: "pub fn previous_block (view : & SideBySide , current : Option < usize >) -> Option < usize >"
---

Returns the block before `current`, wrapping around.

The block before the one at `current` (wraps around); None when there are no blocks.

# Examples

```rust
use crawrad_host::history_view::{previous_block, SideBySide};

let sbs = SideBySide { rows: vec![], blocks: vec![0..1, 2..3] };
assert_eq!(previous_block(&sbs, Some(1)), Some(0));
```
