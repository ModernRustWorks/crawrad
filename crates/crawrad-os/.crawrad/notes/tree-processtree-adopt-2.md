---
id: tree-processtree-adopt-2
kind: doc
target_file: src/tree.rs
line_hint: 76
author: 
created: 
title: "ProcessTree::adopt"
tags: []
signature: "pub fn adopt (child : & Child) -> std :: io :: Result < Self >"
---

Puts `child` and whatever it starts into a new process tree.

Puts `child` (and whatever it starts from now on) in a new tree. Call
it right after `spawn`, before the child had time to start anything.

# Examples

```rust
use crawrad_os::process::no_console;
use crawrad_os::tree::ProcessTree;

let child = no_console("cargo").arg("--version").spawn().unwrap();
let tree = ProcessTree::adopt(&child);
assert!(tree.is_ok());
```

# Errors

Returns an error if creating or assigning the job object fails.
