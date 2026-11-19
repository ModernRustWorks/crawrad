---
id: props-props
kind: doc
target_file: src/props.rs
line_hint: 103
author: 
created: 
title: "Props"
tags: []
signature: "pub struct Props"
---

Centralized catalog of standard properties from classic RAD IDE / CrawRAD.

Similar to the [`crate::Events`] catalog, unifies canonical names,
display labels, semantic types, and categories.

# Examples

```rust
use crawrad_registry::Props;

let prop = Props::name();
assert_eq!(prop.key, "name");
```
