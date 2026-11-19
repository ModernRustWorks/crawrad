---
id: profile-header
kind: doc
target_file: src/profile.rs
line_hint: 17
author: 
created: 
title: "HEADER"
tags: []
signature: "pub const HEADER : & str = \"crawrad-profile 1\""
---

The first line; anything else is not a profile this build can read.

# Examples

```rust
use crawrad_project::profile::HEADER;

assert!(HEADER.starts_with("crawrad-profile"));
```
