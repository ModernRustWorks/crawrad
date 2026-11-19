---
id: package-manager-no-project-hint
kind: doc
target_file: src/package_manager.rs
line_hint: 599
author: 
created: 
title: "NO_PROJECT_HINT"
tags: []
signature: "pub const NO_PROJECT_HINT : & str = \"Open a project or workspace to view and manage its packages.\""
---

The follow-up of [`NO_PROJECT`] — what to do about it.

# Examples

```rust
use crawrad_project::package_manager::NO_PROJECT_HINT;

assert!(!NO_PROJECT_HINT.is_empty());
```
