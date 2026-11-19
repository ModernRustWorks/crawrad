---
id: crash-report-file-name
kind: doc
target_file: src/crash_report.rs
line_hint: 47
author: 
created: 
title: "FILE_NAME"
tags: []
signature: "pub const FILE_NAME : & str = \"crash-report.log\""
---

The crash report file name.

The file name. Unique, so the support instruction fits in one sentence.

# Examples

```rust
use crawrad_os::crash_report::FILE_NAME;

assert_eq!(FILE_NAME, "crash-report.log");
```
