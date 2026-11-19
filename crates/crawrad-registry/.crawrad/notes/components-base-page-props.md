---
id: components-base-page-props
kind: doc
target_file: src/components/base.rs
line_hint: 42
author: 
created: 
title: "page_props"
tags: []
signature: "pub fn page_props () -> Vec < PropDesc >"
---

The page of a `PageControl`: geometry is consequence, not choice.

# Examples

```rust
use crawrad_registry::page_props;

let props = page_props();
assert!(!props.is_empty());
```
