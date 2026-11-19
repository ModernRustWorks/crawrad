---
id: debug-debug-run-without-debugger-once
kind: doc
target_file: src/debug.rs
line_hint: 355
author: 
created: 
title: "Debug::run_without_debugger_once"
tags: []
signature: "pub fn run_without_debugger_once (& mut self)"
---

Sets a flag so the next run proceeds without the debugger attached.

The next `Run` goes without the debugger; asked by `Run without
debugging`, answered once by [`Debug::take_run_without_debugger`].

# Examples

```rust
use crawrad_host::Debug;

let mut debug = Debug::default();
debug.run_without_debugger_once();
assert!(debug.take_run_without_debugger());
```
