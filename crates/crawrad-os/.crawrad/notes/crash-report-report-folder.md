---
id: crash-report-report-folder
kind: doc
target_file: src/crash_report.rs
line_hint: 78
author: 
created: 
title: "report_folder"
tags: []
signature: "pub fn report_folder (configuration : Option < PathBuf >) -> Option < PathBuf >"
---

The folder where the report should go on this machine.

`configuration` is the IDE's configuration folder (`%APPDATA%\CrawRAD`),
passed in by the shell: this crate sits below the one that knows it.

# Examples

```rust
use crawrad_os::crash_report::report_folder;

let folder = report_folder(None);
assert!(folder.is_some());
```
