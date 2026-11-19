---
id: build-progress-buildprogress-feed
kind: doc
target_file: src/build_progress.rs
line_hint: 77
author: 
created: 
title: "BuildProgress::feed"
tags: []
signature: "pub fn feed (& mut self , pipe : crawrad_os :: process :: Pipe , line : & str)"
---

Feeds a single line received from cargo's stdout or stderr.

- Progress bar rewrites on `stderr` (`Building [...] N/M: ...`) update
  `done` and `total`, but are omitted from the log.
- `Compiling `, `Checking ` and `Fresh ` update `current`. `Compiling` and
  `Checking` lines are preserved in the log; `Fresh` lines are not logged.
- Diagnostic JSON lines on `stdout` (`"compiler-artifact"`) increment `done`
  when no progress bar total has been seen yet, and are not logged.
- All other stderr and non-JSON stdout lines are logged.

# Examples

```rust
use crawrad_os::process::Pipe;
use crawrad_project::build_progress::BuildProgress;

let mut progress = BuildProgress::new("Building", "demo", "cargo build");
progress.feed(Pipe::Stderr, "Compiling foo v0.1.0");
assert_eq!(progress.current(), Some("foo v0.1.0"));
```
