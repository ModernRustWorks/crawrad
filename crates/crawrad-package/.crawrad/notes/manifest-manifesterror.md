---
id: manifest-manifesterror
kind: doc
target_file: src/manifest.rs
line_hint: 68
author: 
created: 
title: "ManifestError"
tags: []
signature: "pub enum ManifestError"
---

Why a manifest was refused.

# Examples

```rust
use crawrad_package::ManifestError;

let err = ManifestError::NoComponent;
assert_eq!(err.to_string(), "the manifest declares no component at all");
```
