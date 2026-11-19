---
id: views-extensionview
kind: doc
target_file: src/views.rs
line_hint: 12
author: 
created: 
title: "ExtensionView"
tags: []
signature: "pub struct ExtensionView"
---

A declared view with the tree its extension last sent.

# Examples

```rust
use crawrad_extension_host::views::ExtensionView;

let view = ExtensionView {
    extension: "ext1".into(),
    id: "v1".into(),
    title: "View 1".into(),
    items: Vec::new(),
};
assert_eq!(view.id, "v1");
```
