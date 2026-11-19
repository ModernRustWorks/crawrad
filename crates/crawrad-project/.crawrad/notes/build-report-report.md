---
id: build-report-report
kind: doc
target_file: src/build_report.rs
line_hint: 110
author: 
created: 
title: "Report"
tags: []
signature: "pub struct Report"
---

Description of the completed build used to generate the summary lines.

# Examples

```rust
use crawrad_project::build_report::Report;
use crawrad_project::build_settings::BuildMode;

let rep = Report {
    manifest: "Cargo.toml".to_string(),
    platform: "Host".to_string(),
    configuration: "debug".to_string(),
    executables: vec![],
    mode: BuildMode::Build,
    package: "demo".to_string(),
    success: true,
    errors: 0,
    warnings: 0,
    error_lines: vec![],
};
assert!(rep.success);
```
