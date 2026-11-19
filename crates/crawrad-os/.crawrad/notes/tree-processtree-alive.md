---
id: tree-processtree-alive
kind: doc
target_file: src/tree.rs
line_hint: 82
author: 
created: 
title: "ProcessTree::alive"
tags: []
signature: "pub fn alive (& self) -> u32"
---

How many processes of the tree are running now.

# Examples

```rust
use crawrad_os::process::no_console;
use crawrad_os::tree::ProcessTree;

let child = no_console("cargo").arg("--version").spawn().unwrap();
let tree = ProcessTree::adopt(&child).unwrap();
assert!(tree.alive() >= 0);
```
