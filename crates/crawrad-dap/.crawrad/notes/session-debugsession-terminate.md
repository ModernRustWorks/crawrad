---
id: session-debugsession-terminate
kind: doc
target_file: src/session.rs
line_hint: 458
author: 
created: 
title: "DebugSession::terminate"
tags: []
signature: "pub fn terminate (& mut self) -> Result < () , DapError >"
---

Ends the program and the session.

# Examples

```rust,no_run
// Terminating a session requires an active debug session.
use std::path::Path;
use crawrad_dap::{DebugSession, LaunchConfig};

if let Ok(mut session) = DebugSession::launch(
    Path::new("lldb-dap"),
    &LaunchConfig::default(),
    &std::collections::BTreeMap::<std::path::PathBuf, Vec<crawrad_dap::session::SourceBreakpoint>>::new(),
) {
    assert!(session.terminate().is_ok());
}
```

# Errors

Returns [`DapError`] if the disconnect request fails.
