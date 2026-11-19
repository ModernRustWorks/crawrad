---
id: components-base-font-props
kind: doc
target_file: src/components/base.rs
line_hint: 76
author: 
created: 
title: "font_props"
tags: []
signature: "pub fn font_props () -> Vec < PropDesc >"
---

Properties defining how a control paints its text.

**How a control paints its text** (#433): `Font` and whether it comes
from the parent.

# Examples

```rust
use crawrad_registry::font_props;

let props = font_props();
assert_eq!(props.len(), 2);
```
