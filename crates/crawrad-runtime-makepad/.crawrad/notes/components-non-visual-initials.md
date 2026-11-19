---
id: components-non-visual-initials
kind: doc
target_file: src/components/non_visual.rs
line_hint: 43
author: 
created: 
title: "initials"
tags: []
signature: "pub fn initials (class : & str) -> String"
---

A class's initials: its UPPERCASE letters, in the order they appear.

`ImageList` -> `IL`, `ActionList` -> `AL`, `Timer` -> `T`. It is the same
computation as the RXP's (`crawrad-runtime-egui/src/lib.rs:2399`), and it is
pure on purpose: what the badge writes can be measured without a window.

# Examples

```rust
use crawrad_runtime_makepad::components::non_visual::initials;

assert_eq!(initials("ImageList"), "IL");
assert_eq!(initials("ActionList"), "AL");
```
