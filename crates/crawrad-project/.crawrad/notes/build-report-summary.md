---
id: build-report-summary
kind: doc
target_file: src/build_report.rs
line_hint: 125
author: 
created: 
title: "summary"
tags: []
signature: "pub fn summary (r : & Report) -> Vec < String >"
---

Formats the summary block (Project, Platform, Output, Status, Result, Errors).

# Examples

```rust
use crawrad_project::build_report::{summary, Report};
use crawrad_project::build_settings::BuildMode;

let rep = Report {
    manifest: "Cargo.toml".to_string(),
    platform: "Host".to_string(),
    configuration: "debug".to_string(),
    executables: vec![],
    mode: BuildMode::Check,
    package: "demo".to_string(),
    success: true,
    errors: 0,
    warnings: 0,
    error_lines: vec![],
};
let lines = summary(&rep);
assert!(!lines.is_empty());
```
