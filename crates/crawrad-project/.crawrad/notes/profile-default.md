---
id: profile-default
kind: doc
target_file: src/profile.rs
line_hint: 20
author: 
created: 
title: "DEFAULT"
tags: []
signature: "pub const DEFAULT : & str = \"Default\""
---

The built-in profile: the IDE as it ships, with no file behind it.

# Examples

```rust
use crawrad_project::profile::DEFAULT;

assert_eq!(DEFAULT, "Default");
```
