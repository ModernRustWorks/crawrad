---
id: package-manager-no-results
kind: doc
target_file: src/package_manager.rs
line_hint: 591
author: 
created: 
title: "no_results"
tags: []
signature: "pub fn no_results (query : & str) -> String"
---

What it says when the search came back empty.

# Examples

```rust
use crawrad_project::package_manager::no_results;

assert_eq!(no_results("foo"), "No crates found matching 'foo'");
```
