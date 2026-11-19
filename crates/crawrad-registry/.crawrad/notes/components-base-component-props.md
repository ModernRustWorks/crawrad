---
id: components-base-component-props
kind: doc
target_file: src/components/base.rs
line_hint: 11
author: 
created: 
title: "component_props"
tags: []
signature: "pub fn component_props () -> Vec < PropDesc >"
---

What exists in **every** component, visual or not: the name.

# Examples

```rust
use crawrad_registry::component_props;

let props = component_props();
assert_eq!(props.len(), 1);
```
