---
id: profile-names
kind: doc
target_file: src/profile.rs
line_hint: 65
author: 
created: 
title: "names"
tags: []
signature: "pub fn names (dir : & Path) -> Vec < String >"
---

[`DEFAULT`] first, then every saved profile in name order.

# Examples

```rust
use std::path::Path;
use crawrad_project::profile::{DEFAULT, names};

let list = names(Path::new("non_existent_profile_dir_xyz"));
assert_eq!(list, vec![DEFAULT]);
```
