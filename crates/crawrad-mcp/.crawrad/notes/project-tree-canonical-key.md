---
id: project-tree-canonical-key
kind: doc
target_file: src/project_tree.rs
line_hint: 51
author: 
created: 
title: "canonical_key"
tags: []
signature: "pub fn canonical_key (path_str : & str) -> String"
---

A path as the read/edit hashes are keyed: `/` separators, no leading `./`.

# Examples

```rust
use crawrad_mcp::project_tree::canonical_key;

assert_eq!(canonical_key(r".\src\main.rs"), "src/main.rs");
```
