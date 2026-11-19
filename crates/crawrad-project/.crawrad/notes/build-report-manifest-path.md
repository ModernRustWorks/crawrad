---
id: build-report-manifest-path
kind: doc
target_file: src/build_report.rs
line_hint: 11
author: 
created: 
title: "manifest_path"
tags: []
signature: "pub fn manifest_path (root : Option < & Path > , package : & str , artifacts : & [CompiledArtifact]) -> String"
---

Determines the canonical manifest path for the compiled package.

# Examples

```rust
use crawrad_project::build_report::manifest_path;

let path = manifest_path(None, "demo", &[]);
assert_eq!(path, "demo/Cargo.toml");
```
