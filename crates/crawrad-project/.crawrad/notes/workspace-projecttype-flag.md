---
id: workspace-projecttype-flag
kind: doc
target_file: src/workspace.rs
line_hint: 1380
author: 
created: 
title: "ProjectType::flag"
tags: []
signature: "pub fn flag (self) -> & 'static str"
---

The flag that decides what `cargo new` produces.

Explicit in all three, not just library: `--bin` is cargo's default today,
and relying on a default that can change means relying on something not written
anywhere in our codebase.

# Examples

```rust
use crawrad_project::workspace::ProjectType;

assert_eq!(ProjectType::Library.flag(), "--lib");
```
