---
id: icon-strip-extra-icons
kind: doc
target_file: src/icon_strip.rs
line_hint: 18
author: 
created: 
title: "extra_icons"
tags: []
signature: "pub fn extra_icons (reg : & crawrad_registry :: Registry) -> Vec < (String , Vec < IconStep >) >"
---

Collects third-party icons from registry classes not in `ClassIcon`.

Collects third-party icons from the registry for classes not covered by `ClassIcon`.

# Examples

```rust
use crawrad_registry::Registry;
use crawrad_project::icon_strip::extra_icons;

let reg = Registry::builtin();
let extras = extra_icons(&reg);
assert!(extras.is_empty());
```
