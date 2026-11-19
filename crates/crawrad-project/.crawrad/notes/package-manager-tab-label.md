---
id: package-manager-tab-label
kind: doc
target_file: src/package_manager.rs
line_hint: 575
author: 
created: 
title: "Tab::label"
tags: []
signature: "pub fn label (self , installed : Option < usize >) -> String"
---

Returns what the tab button says.

What the tab button says. `Installed` carries the count, which is the
only number on this screen that answers a question before it is asked.
`None` is "not read yet", and the tab says no number instead of a
false zero.

# Examples

```rust
use crawrad_project::package_manager::Tab;

assert_eq!(Tab::Browse.label(None), "Browse (crates.io)");
assert_eq!(Tab::Installed.label(Some(3)), "Installed (3)");
```
