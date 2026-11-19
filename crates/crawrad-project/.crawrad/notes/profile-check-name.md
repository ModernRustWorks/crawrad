---
id: profile-check-name
kind: doc
target_file: src/profile.rs
line_hint: 46
author: 
created: 
title: "check_name"
tags: []
signature: "pub fn check_name (name : & str) -> Result < & str , String >"
---

The name as typed, trimmed, or why it cannot name a saved profile.

# Errors

Returns an error message if the name is empty, matches [`DEFAULT`], starts
with `.`, exceeds [`NAME_MAX`], or contains disallowed characters.

# Examples

```rust
use crawrad_project::profile::check_name;

assert_eq!(check_name("Custom"), Ok("Custom"));
assert!(check_name("Default").is_err());
```
