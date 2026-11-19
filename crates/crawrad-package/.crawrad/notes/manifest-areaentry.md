---
id: manifest-areaentry
kind: doc
target_file: src/manifest.rs
line_hint: 239
author: 
created: 
title: "AreaEntry"
tags: []
signature: "pub struct AreaEntry"
---

The area of a drawing verb in the manifest file format.

**THE AREA of a verb, in the file format.**

```toml
area = { inset = 4.0 }                                # recessed
area = { fraction_of = "value", axis = "x", max = 100.0 }   # a bar
```

Two forms, and the second is what made this section exist: a gauge that
shows a VALUE did not fit into five verbs that only knew how to paint the entire
box.

# Examples

```rust
use crawrad_package::manifest::AreaEntry;

let area = AreaEntry {
    inset: Some(4.0),
    fraction_of: None,
    axis: None,
    min: None,
    max: None,
};
assert_eq!(area.inset, Some(4.0));
```
