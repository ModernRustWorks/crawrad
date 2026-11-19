---
id: explorer-rows-rows-member-member-with-problems
kind: doc
target_file: src/explorer_rows/rows/member.rs
line_hint: 55
author: 
created: 
title: "Member::with_problems"
tags: []
signature: "pub fn with_problems (mut self , problems : Problems) -> Self"
---

The problems of every file under the folder: they colour the node.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_rows::rows::member::Member;
use crawrad_project::explorer_rows::problems::Problems;
use crawrad_project::explorer_rows::row::ExplorerRow;

let m = Member::new(Path::new("app"), "app", None, true)
    .with_problems(Problems { errors: 1, warnings: 0 });
assert!(m.badge().contains("1"));
```
