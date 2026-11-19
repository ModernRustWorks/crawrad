---
id: navigation-tab-icon
kind: doc
target_file: src/navigation.rs
line_hint: 283
author: 
created: 
title: "tab_icon"
tags: []
signature: "pub fn tab_icon (file : & Path , roots : & [PathBuf] , generated_suffix : & str) -> & 'static str"
---

Returns the display icon string for a tab based on its file type.

The mark in front of a tab's name: a padlock when [`locked`], the crab
for Rust, a page for anything else.

# Examples

```rust
use crawrad_host::navigation::tab_icon;
use std::path::Path;

assert_eq!(tab_icon(Path::new("main.rs"), &[], ".designer.rs"), "\u{1F980} ");
```
