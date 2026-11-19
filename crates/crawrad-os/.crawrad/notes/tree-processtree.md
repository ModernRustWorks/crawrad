---
id: tree-processtree
kind: doc
target_file: src/tree.rs
line_hint: 20
author: 
created: 
title: "ProcessTree"
tags: []
signature: "pub struct ProcessTree"
---

The job a child and its descendants run in.

# Examples

```rust
use crawrad_os::process::no_console;
use crawrad_os::tree::ProcessTree;

let child = no_console("cargo").arg("--version").spawn().unwrap();
let tree = ProcessTree::adopt(&child).unwrap();
let _ = tree.alive();
tree.end();
```
