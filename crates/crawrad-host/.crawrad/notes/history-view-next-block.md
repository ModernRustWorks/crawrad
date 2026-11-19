---
id: history-view-next-block
kind: doc
target_file: src/history_view.rs
line_hint: 172
author: 
created: 
title: "next_block"
tags: []
signature: "pub fn next_block (view : & SideBySide , current : Option < usize >) -> Option < usize >"
---

Returns the block after `current`, wrapping around.

The block after the one at `current` (wraps around); None when there are no blocks.

# Examples

```rust
use crawrad_host::history_view::{next_block, SideBySide};

let sbs = SideBySide { rows: vec![], blocks: vec![0..1, 2..3] };
assert_eq!(next_block(&sbs, Some(0)), Some(1));
```
