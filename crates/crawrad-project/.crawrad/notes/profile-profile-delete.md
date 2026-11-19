---
id: profile-profile-delete
kind: doc
target_file: src/profile.rs
line_hint: 180
author: 
created: 
title: "Profile::delete"
tags: []
signature: "pub fn delete (dir : & Path , name : & str) -> Result < () , String >"
---

Deletes the saved profile `name`.

# Errors

Returns an error message if the profile cannot be found or deleted.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_project::profile::Profile;

// Avoid deleting files during doctests.
assert!(Profile::delete(Path::new("profiles"), "custom").is_ok());
```
