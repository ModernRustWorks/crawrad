---
id: manifest-iconentry
kind: doc
target_file: src/manifest.rs
line_hint: 375
author: 
created: 
title: "IconEntry"
tags: []
signature: "pub struct IconEntry"
---

An icon step in the manifest file format.

**AN ICON STEP, in the file format.**

Follows the same grammar as [`DrawEntry`]: **bare name, and the payload in
neighbouring fields.**

```toml
[[component.icon]]
step = "move_to"
x = 4.0
y = 3.5

[[component.icon]]
step = "circle"
x = 8.0
y = 8.0
r = 3.0

[[component.icon]]
step = "fill"
```

# Examples

```rust
use crawrad_package::manifest::IconEntry;

let icon = IconEntry {
    step: "move_to".into(),
    x: Some(4.0),
    y: Some(3.5),
    w: None,
    h: None,
    r: None,
};
assert_eq!(icon.step, "move_to");
```
