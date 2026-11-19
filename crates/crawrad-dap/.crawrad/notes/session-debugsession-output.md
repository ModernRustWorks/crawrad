---
id: session-debugsession-output
kind: doc
target_file: src/session.rs
line_hint: 223
author: 
created: 
title: "DebugSession::output"
tags: []
signature: "pub fn output (& self) -> & [String]"
---

What the program printed so far (`output` events), in order.

# Examples

```rust,no_run
// A live session requires an active DAP adapter handshake.
use std::collections::BTreeMap;
use std::path::Path;
use crawrad_dap::{DebugSession, LaunchConfig, SourceBreakpoint};

if let Ok(session) = DebugSession::launch(
    Path::new("lldb-dap"),
    &LaunchConfig::default(),
    &BTreeMap::<std::path::PathBuf, Vec<crawrad_dap::session::SourceBreakpoint>>::new(),
) {
    assert!(session.output().is_empty());
}
```
