---
id: package-manager-search-hint
kind: doc
target_file: src/package_manager.rs
line_hint: 588
author: 
created: 
title: "SEARCH_HINT"
tags: []
signature: "pub const SEARCH_HINT : & str = \"Type a crate name above and press Enter to search.\""
---

What the `Browse` tab says before anybody has searched.

# Examples

```rust
use crawrad_project::package_manager::SEARCH_HINT;

assert!(!SEARCH_HINT.is_empty());
```
