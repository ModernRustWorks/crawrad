---
id: diagnostics-model-compiledartifact
kind: doc
target_file: src/diagnostics_model.rs
line_hint: 207
author: 
created: 
title: "CompiledArtifact"
tags: []
signature: "pub struct CompiledArtifact"
---

Compiled artifact info emitted by cargo.

# Examples

```rust
use crawrad_project::diagnostics_model::CompiledArtifact;

let art = CompiledArtifact {
    package_name: "demo".to_string(),
    kind: vec!["bin".to_string()],
    manifest_path: None,
    executable: None,
    filenames: vec![],
};
assert_eq!(art.package_name, "demo");
```
