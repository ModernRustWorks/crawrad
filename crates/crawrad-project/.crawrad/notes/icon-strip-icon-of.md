---
id: icon-strip-icon-of
kind: doc
target_file: src/icon_strip.rs
line_hint: 37
author: 
created: 
title: "icon_of"
tags: []
signature: "pub fn icon_of (d : & crawrad_registry :: ComponentDesc) -> Option < Vec < IconStep > >"
---

The palette icon of one catalogue class, or None when it has none.

The built-in `ClassIcon` wins; then the manifest's own `icon`; then, for an
inheriting class with no icon of its own, its base's (#934). The palette strip
and the non-visual badge on the canvas both ask here, so they draw alike.

# Examples

```rust
use crawrad_registry::Registry;
use crawrad_project::icon_strip::icon_of;

let reg = Registry::builtin();
let timer = reg.all().find(|d| d.class == "Timer").expect("Timer is built in");
assert!(icon_of(timer).is_some());
```
