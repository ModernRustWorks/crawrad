---
id: installation-verifiedpackage
kind: doc
target_file: src/installation.rs
line_hint: 80
author: 
created: 
title: "VerifiedPackage"
tags: []
signature: "pub struct VerifiedPackage"
---

What verification established about a package — before touching disk.

# Examples

```rust
use crawrad_project::installation::VerifiedPackage;

let vp = VerifiedPackage {
    package: "my_package".to_string(),
    classes: vec!["Gauge".to_string()],
};
assert_eq!(vp.package, "my_package");
```
