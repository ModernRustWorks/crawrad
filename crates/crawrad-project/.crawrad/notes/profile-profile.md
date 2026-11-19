---
id: profile-profile
kind: doc
target_file: src/profile.rs
line_hint: 26
author: 
created: 
title: "Profile"
tags: []
signature: "pub struct Profile"
---

Everything a profile holds; `None` (and an empty dock) is the default.

# Examples

```rust
use crawrad_project::profile::Profile;

let p = Profile::default();
assert!(p.dock.is_empty());
```
