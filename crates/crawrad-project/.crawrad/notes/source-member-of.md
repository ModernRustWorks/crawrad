---
id: source-member-of
kind: doc
target_file: src/source.rs
line_hint: 91
author: 
created: 
title: "member_of"
tags: []
signature: "pub fn member_of (form : & Path) -> Option < PathBuf >"
---

The member folder a form belongs to — `<member>/src/<form>.crawform`.

`None` when the form is not inside a `src/`, which is the layout the whole
IDE assumes ([ADR 0017]).


# Examples

```rust
use std::path::{Path, PathBuf};
use crawrad_project::source::member_of;

assert_eq!(member_of(Path::new("app/src/main.crawform")), Some(PathBuf::from("app")));
assert_eq!(member_of(Path::new("app/main.crawform")), None);
```
