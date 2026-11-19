---
id: dependencies-component-crate-path
kind: doc
target_file: src/dependencies.rs
line_hint: 179
author: 
created: 
title: "component_crate_path"
tags: []
signature: "fn component_crate_path (name : & str) -> PathBuf"
---

Returns the path to a component suite crate.

A crate of the component suite (`components/<name>`), found the way the
workspace crates are: `components/` is the sibling of `crates/`.
