---
id: manifest-manifest-to-components
kind: doc
target_file: src/manifest.rs
line_hint: 869
author: 
created: 
title: "Manifest::to_components"
tags: []
signature: "pub fn to_components (& self) -> Result < Vec < ExternalComponent > , ManifestError >"
---

The components of this package, in the form the catalogue knows how to receive.

**It does not register anything.** Whoever decides if a component enters is
`Registry::register_external`, which has the seven refusals — and one can
load the manifest without touching any catalogue, which is what the store
does to show what a package brings before installing.

# Examples

```rust
use crawrad_package::{Manifest, PackageId};
use crawrad_package::manifest::SDK;

let m = Manifest {
    sdk: SDK,
    doc: String::new(),
    package: PackageId { name: "pkg".into(), version: "0.1".into(), description: "".into() },
    components: Vec::new(),
};
let comps = m.to_components().unwrap();
assert!(comps.is_empty());
```

# Errors

Returns [`ManifestError`](crate::ManifestError) if any component definition is invalid.
