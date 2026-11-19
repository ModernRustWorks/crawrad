---
id: diff-review-canonical-path-key
kind: doc
target_file: src/diff_review.rs
line_hint: 164
author: 
created: 
title: "canonical_path_key"
tags: []
signature: "pub fn canonical_path_key (path : & Path) -> String"
---

Computes the canonical key used to group file paths.

# Examples

```rust
use crawrad_agent::diff_review::canonical_path_key;
use std::path::Path;

assert_eq!(canonical_path_key(Path::new("./src\\main.rs")), "src/main.rs");
```
