---
id: profile-profile-decode
kind: doc
target_file: src/profile.rs
line_hint: 133
author: 
created: 
title: "Profile::decode"
tags: []
signature: "pub fn decode (text : & str) -> Option < Self >"
---

Back from the text; `None` when the header is not this format's.

# Examples

```rust
use crawrad_project::profile::Profile;

let p = Profile::decode("crawrad-profile 1\ndock\n").unwrap();
assert_eq!(p.dock, "");
```
