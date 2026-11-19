---
id: props-props-tab-stop
kind: doc
target_file: src/props.rs
line_hint: 411
author: 
created: 
title: "Props::tab_stop"
tags: []
signature: "pub fn tab_stop (default : bool) -> PropDesc"
---

Whether Tab stops on the control (`TabStop`, #667).

Whether Tab stops on the control (`TabStop`, #667): off, the user
still reaches it with the mouse, and Tab passes it by.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::tab_stop(true);
assert_eq!(p.key, "tab_stop");
```
