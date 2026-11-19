---
id: link-pick-linkpick-answered
kind: doc
target_file: src/link_pick.rs
line_hint: 104
author: 
created: 
title: "LinkPick::answered"
tags: []
signature: "pub fn answered (& mut self , ticket : u64 , found : Vec < Linkable >) -> bool"
---

The answer to `ticket` arrived; whether it was the newest question.

# Examples

```rust
use crawrad_host::link_pick::LinkPick;
use std::path::Path;

let mut pick = LinkPick::default();
pick.open(0, Path::new("src"));
let (ticket, _) = pick.ask("term").unwrap();
assert!(pick.answered(ticket, vec![]));
```
