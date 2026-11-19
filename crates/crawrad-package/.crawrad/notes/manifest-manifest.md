---
id: manifest-manifest
kind: doc
target_file: src/manifest.rs
line_hint: 188
author: 
created: 
title: "Manifest"
tags: []
signature: "pub struct Manifest"
---

The entire file.

# Examples

```rust
use crawrad_package::manifest::SDK;
use crawrad_package::{Manifest, ManifestError};

// A package with no component is refused: there is nothing to install.
let json = format!(r#"{{"sdk": {SDK}, "package": {{"name": "test"}}, "components": []}}"#);
assert_eq!(Manifest::parse(&json), Err(ManifestError::NoComponent));
```
