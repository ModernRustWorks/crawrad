---
id: session-debugsession-pause
kind: doc
target_file: src/session.rs
line_hint: 449
author: 
created: 
title: "DebugSession::pause"
tags: []
signature: "pub fn pause (& mut self) -> Result < () , DapError >"
---

Asks a running program to stop; the `stopped` event says when it did.

# Examples

```rust,no_run
// Pausing execution requires an active debug session.
use std::path::Path;
use crawrad_dap::{DebugSession, LaunchConfig};

if let Ok(mut session) = DebugSession::launch(
    Path::new("lldb-dap"),
    &LaunchConfig::default(),
    &std::collections::BTreeMap::<std::path::PathBuf, Vec<crawrad_dap::session::SourceBreakpoint>>::new(),
) {
    let _ = session.pause();
}
```

# Errors

Returns [`DapError`] if querying threads or sending the pause request fails.
