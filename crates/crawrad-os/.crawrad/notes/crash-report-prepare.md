---
id: crash-report-prepare
kind: doc
target_file: src/crash_report.rs
line_hint: 68
author: 
created: 
title: "prepare"
tags: []
signature: "pub fn prepare (error : & str , details : & str , folder : Option < & Path >) -> Report"
---

Builds the report and writes it in `folder`.

`folder` is a parameter so the test can pass a temporary directory and read
what was written. `None`, or a folder that refuses writes, is the path where
only the message remains, and it is still complete.

# Examples

```rust
use crawrad_os::crash_report::prepare;

let r = prepare("err", "details", None);
assert_eq!(r.code, 1);
```
