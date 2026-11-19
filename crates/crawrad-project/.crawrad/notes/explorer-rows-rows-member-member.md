---
id: explorer-rows-rows-member-member
kind: doc
target_file: src/explorer_rows/rows/member.rs
line_hint: 27
author: 
created: 
title: "Member"
tags: []
signature: "pub struct Member"
---

One member of the group.

# Examples

```rust
use std::path::Path;
use crawrad_project::explorer_rows::rows::member::Member;
use crawrad_project::explorer_rows::row::ExplorerRow;

let m = Member::new(Path::new("app"), "app", Some("active"), true);
assert_eq!(m.text(), "app");
```
