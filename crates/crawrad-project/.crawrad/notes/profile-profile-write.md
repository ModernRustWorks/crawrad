---
id: profile-profile-write
kind: doc
target_file: src/profile.rs
line_hint: 172
author: 
created: 
title: "Profile::write"
tags: []
signature: "pub fn write (& self , dir : & Path , name : & str) -> Result < () , String >"
---

Writes it as `name`, replacing a profile of that name.

# Errors

Returns an error message if the directory cannot be created or the file written.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_project::profile::Profile;

// Avoid writing files during doctests.
let p = Profile::default();
assert!(p.write(Path::new("profiles"), "custom").is_ok());
```
