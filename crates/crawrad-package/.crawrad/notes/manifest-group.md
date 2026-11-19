---
id: manifest-group
kind: doc
target_file: src/manifest.rs
line_hint: 756
author: 
created: 
title: "Group"
tags: []
signature: "pub enum Group"
---

Where the property appears in the Object Inspector.

The file names are the ones the Inspector SHOWS, and not those of the enum in
Rust: whoever writes the manifest reads the screen, not this code.

# Examples

```rust
use crawrad_package::manifest::Group;

let g = Group::Identity;
assert_eq!(g, Group::Identity);
```
