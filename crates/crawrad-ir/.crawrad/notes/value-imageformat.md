---
id: value-imageformat
kind: doc
target_file: src/value.rs
line_hint: 1429
author: 
created: 
title: "ImageFormat"
tags: []
signature: "pub enum ImageFormat"
---

Supported graphic formats in the `.crawform` document.

# Examples

```rust
use crawrad_ir::ImageFormat;

assert_eq!(ImageFormat::default(), ImageFormat::Unknown);
assert_eq!(ImageFormat::Png.as_str(), "png");
```
