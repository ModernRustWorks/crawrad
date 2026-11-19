---
id: profile-profile-encode
kind: doc
target_file: src/profile.rs
line_hint: 112
author: 
created: 
title: "Profile::encode"
tags: []
signature: "pub fn encode (& self) -> String"
---

The text of the file.

# Examples

```rust
use crawrad_project::profile::Profile;

let p = Profile::default();
assert!(p.encode().starts_with("crawrad-profile 1"));
```
