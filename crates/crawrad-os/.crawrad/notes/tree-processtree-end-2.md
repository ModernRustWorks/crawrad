---
id: tree-processtree-end-2
kind: doc
target_file: src/tree.rs
line_hint: 115
author: 
created: 
title: "ProcessTree::end"
tags: []
signature: "pub fn end (& self)"
---

Ends every process of the tree, the root and all it started.

# Examples

```rust
use crawrad_os::process::no_console;
use crawrad_os::tree::ProcessTree;

let child = no_console("cargo").arg("--version").spawn().unwrap();
let tree = ProcessTree::adopt(&child).unwrap();
tree.end();
```
