---
id: crash-report-report
kind: doc
target_file: src/crash_report.rs
line_hint: 56
author: 
created: 
title: "Report"
tags: []
signature: "pub struct Report"
---

What the IDE managed to say about the failure.

Separate from whoever shows the box, so it can be measured without a
screen: a fabricated error goes in as text.

# Examples

```rust
use crawrad_os::crash_report::Report;

let r = Report { message: "msg".to_string(), file: None, code: 1 };
assert_eq!(r.code, 1);
```
