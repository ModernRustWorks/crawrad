---
id: components-page-control-pages-top
kind: doc
target_file: src/components/page_control.rs
line_hint: 78
author: 
created: 
title: "pages_top"
tags: []
signature: "pub fn pages_top (bottom : bool) -> f32"
---

The top coordinate of the pages' client area.

The top of the pages' area: below the strip, or at the top when the
strip is at the bottom. The area is `h - strip` tall either way.

# Examples

```rust
use crawrad_registry::{TAB_STRIP_HEIGHT, pages_top};

assert_eq!(pages_top(false), TAB_STRIP_HEIGHT);
```
