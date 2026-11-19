---
id: profile-all
kind: doc
target_file: src/profile.rs
line_hint: 102
author: 
created: 
title: "all"
tags: []
signature: "pub fn all () -> Vec < String >"
---

[`names`] in the profiles' own [`folder`].

# Examples

```rust
use crawrad_project::profile::{DEFAULT, all};

let list = all();
assert!(list.contains(&DEFAULT.to_string()));
```
