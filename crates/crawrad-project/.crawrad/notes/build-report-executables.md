---
id: build-report-executables
kind: doc
target_file: src/build_report.rs
line_hint: 57
author: 
created: 
title: "executables"
tags: []
signature: "pub fn executables (root : Option < & Path > , package : & str , profile : BuildProfile , targets : & [TargetPlatform] , artifacts : & [CompiledArtifact] ,) -> Vec < PathBuf >"
---

Locates output binaries/executables, checking artifacts and cache fallbacks.

# Examples

```rust
use crawrad_project::build_report::executables;
use crawrad_project::build_settings::BuildProfile;

let exes = executables(None, "demo", BuildProfile::Debug, &[], &[]);
assert!(exes.is_empty());
```
