---
id: crash-report-report-2
kind: doc
target_file: src/crash_report.rs
line_hint: 149
author: 
created: 
title: "report"
tags: []
signature: "pub fn report (error : & str , details : & str , configuration : Option < PathBuf > , show_box : bool) -> u8"
---

Executes the failure path, writes the report, and returns exit code.

The whole failure path: write, tell, and return the exit code.

Called from one place only, the panic hook in the shell's `main.rs`.
`show_box` is off in debug builds, where the console is there and a modal
box on every panic would get in the way.

# Examples

```rust
use crawrad_os::crash_report::report;

let code = report("test error", "details", None, false);
assert_eq!(code, 1);
```
