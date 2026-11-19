---
id: history-entry
kind: doc
target_file: src/history.rs
line_hint: 39
author: 
created: 
title: "Entry"
tags: []
signature: "pub struct Entry"
---

One recorded local file history entry.

# Examples

```rust
use crawrad_host::history::{Entry, Origin};

let entry = Entry { time: 1000, hash: "abc".into(), origin: Origin::Save };
assert_eq!(entry.time, 1000);
```
