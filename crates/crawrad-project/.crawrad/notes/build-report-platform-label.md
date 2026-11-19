---
id: build-report-platform-label
kind: doc
target_file: src/build_report.rs
line_hint: 32
author: 
created: 
title: "platform_label"
tags: []
signature: "pub fn platform_label (targets : & [TargetPlatform]) -> String"
---

Human-readable label for selected compilation targets.

# Examples

```rust
use crawrad_project::build_report::platform_label;

let label = platform_label(&[]);
assert!(!label.is_empty());
```
