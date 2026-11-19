---
id: build-settings-targetplatform-check-list
kind: doc
target_file: src/build_settings.rs
line_hint: 69
author: 
created: 
title: "TargetPlatform::check_list"
tags: []
signature: "pub fn check_list (selected : & [Self]) -> (Vec < String > , Vec < String > , Vec < bool >)"
---

Returns checklist rows, triples, and selection states for all targets.

The Build dialog's CheckListBox (#604), in `ALL`'s order: each row's
text (mark and label), its detail (the triple) and whether it is in
`selected`.

# Examples

```rust
use crawrad_project::build_settings::TargetPlatform;

let (rows, triples, on) = TargetPlatform::check_list(&[TargetPlatform::Linux64]);
assert_eq!(rows.len(), TargetPlatform::ALL.len());
assert_eq!(triples.len(), TargetPlatform::ALL.len());
assert_eq!(on.len(), TargetPlatform::ALL.len());
```
