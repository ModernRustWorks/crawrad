---
id: recover-mirror-follow
kind: doc
target_file: src/recover.rs
line_hint: 42
author: 
created: 
title: "Mirror::follow"
tags: []
signature: "pub fn follow (& mut self , root : Option < & Path >) -> Option < Vec < Pending > >"
---

Follows the open project and returns pending copies waiting for an answer.

Follows the open project. Leaving one brings its folder down to the
copies still waiting; arriving at one keeps what a crash left there,
and answers it — `None` when the project did not change.

# Examples

```rust
use crawrad_project::recover::Mirror;

let mut mirror = Mirror::default();
assert_eq!(mirror.follow(None), None);
```
