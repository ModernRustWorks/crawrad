---
id: build-settings-page-title
kind: doc
target_file: src/build_settings.rs
line_hint: 181
author: 
created: 
title: "page_title"
tags: []
signature: "pub fn page_title (m : BuildMode) -> & 'static str"
---

The display title for each build command.

# Examples

```rust
use crawrad_project::build_settings::{page_title, BuildMode};

assert_eq!(page_title(BuildMode::Build), "Build");
```
