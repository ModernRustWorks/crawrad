---
id: components-link-label-link-label
kind: doc
target_file: src/components/link_label.rs
line_hint: 5
author: 
created: 
title: "link_label"
tags: []
signature: "pub fn link_label () -> ComponentDesc"
---

A label drawn as a clickable hyperlink.

`LinkLabel` (#500, wave 1): a caption drawn as a link; a click opens its
`URL` in the system's browser, then runs `OnClick`.

# Examples

```rust
use crawrad_registry::link_label;

let desc = link_label();
assert_eq!(desc.class, "LinkLabel");
```
