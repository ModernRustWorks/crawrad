---
id: manifest-manifest-to-json
kind: doc
target_file: src/manifest.rs
line_hint: 864
author: 
created: 
title: "Manifest::to_json"
tags: []
signature: "pub fn to_json (& self) -> String"
---

The manifest as text, in the form that `New Package` writes.

# Examples

```rust
use crawrad_package::{Manifest, PackageId};
use crawrad_package::manifest::SDK;

let m = Manifest {
    sdk: SDK,
    doc: String::new(),
    package: PackageId { name: "p".into(), version: "1".into(), description: "".into() },
    components: Vec::new(),
};
assert!(m.to_json().contains("\"sdk\""));
```

# Panics

Panics if serialization to pretty JSON fails unexpectedly.
