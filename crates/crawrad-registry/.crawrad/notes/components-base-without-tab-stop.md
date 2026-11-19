---
id: components-base-without-tab-stop
kind: doc
target_file: src/components/base.rs
line_hint: 62
author: 
created: 
title: "without_tab_stop"
tags: []
signature: "pub fn without_tab_stop (props : & mut Vec < PropDesc >)"
---

Removes `tab_stop` from a component's property list.

#139: the container itself never takes the focus, so no target can honour a TabStop on it;
its TabOrder still orders the controls inside it.

# Examples

```rust
use crawrad_registry::{window_props, without_tab_stop};

let mut props = window_props();
without_tab_stop(&mut props);
assert!(!props.iter().any(|p| p.key == "tab_stop"));
```
