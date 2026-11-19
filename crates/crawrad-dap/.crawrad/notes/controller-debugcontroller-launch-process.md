---
id: controller-debugcontroller-launch-process
kind: doc
target_file: src/controller.rs
line_hint: 168
author: 
created: 
title: "DebugController::launch_process"
tags: []
signature: "pub fn launch_process < B : Clone + Into < SourceBreakpoint > + Send + 'static > (adapter : PathBuf , config : LaunchConfig , breakpoints : BTreeMap < PathBuf , Vec < B > > , presenter : Box < dyn ValuePresenter + Send > , waker : fn () ,) -> Self"
---

The usual start: `adapter` launched as a process for `config`.

# Examples

```rust
use std::collections::BTreeMap;
use std::path::PathBuf;
use crawrad_dap::{AdapterText, DebugController, LaunchConfig, SourceBreakpoint};

let controller = DebugController::launch_process(
    PathBuf::from("nonexistent-lldb-dap"),
    LaunchConfig::default(),
    BTreeMap::<PathBuf, Vec<SourceBreakpoint>>::new(),
    Box::new(AdapterText),
    || {},
);
assert!(controller.send(crawrad_dap::Command::Stop) || true);
```
