---
id: session-debugsession-locals
kind: doc
target_file: src/session.rs
line_hint: 491
author: 
created: 
title: "DebugSession::locals"
tags: []
signature: "pub fn locals (& self , frame_id : i64) -> Result < Vec < Variable > , DapError >"
---

Retrieves local variables for a stack frame.

The locals of a frame: its first scope, which is the one every adapter
puts first (`Locals`).

# Examples

```rust,no_run
// Querying locals requires an active debug session and valid frame ID.
use std::path::Path;
use crawrad_dap::{DebugSession, LaunchConfig};

if let Ok(session) = DebugSession::launch(
    Path::new("lldb-dap"),
    &LaunchConfig::default(),
    &std::collections::BTreeMap::<std::path::PathBuf, Vec<crawrad_dap::session::SourceBreakpoint>>::new(),
) {
    let _ = session.locals(1);
}
```

# Errors

Returns [`DapError`] if scopes or variables cannot be retrieved.
