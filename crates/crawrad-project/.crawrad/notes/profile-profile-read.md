---
id: profile-profile-read
kind: doc
target_file: src/profile.rs
line_hint: 162
author: 
created: 
title: "Profile::read"
tags: []
signature: "pub fn read (dir : & Path , name : & str) -> Result < Self , String >"
---

The profile `name` in `dir`: [`DEFAULT`] is always there.

# Errors

Returns an error message if the file cannot be read or decoded.

# Examples

```rust
use std::path::Path;
use crawrad_project::profile::{DEFAULT, Profile};

let p = Profile::read(Path::new("."), DEFAULT).unwrap();
assert_eq!(p, Profile::default());
```
