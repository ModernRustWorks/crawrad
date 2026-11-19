---
id: session-debugsession-set-breakpoints
kind: doc
target_file: src/session.rs
line_hint: 236
author: 
created: 
title: "DebugSession::set_breakpoints"
tags: []
signature: "pub fn set_breakpoints (& mut self , path : & Path , wanted : & [SourceBreakpoint] ,) -> Result < Vec < Breakpoint > , DapError >"
---

Replaces the breakpoints of `path` (DAP sets them per file).

# Examples

```rust,no_run
// Modifying breakpoints requires an active debug session.
use std::path::Path;
use crawrad_dap::{DebugSession, LaunchConfig};

if let Ok(mut session) = DebugSession::launch(
    Path::new("lldb-dap"),
    &LaunchConfig::default(),
    &std::collections::BTreeMap::<std::path::PathBuf, Vec<crawrad_dap::session::SourceBreakpoint>>::new(),
) {
    assert!(session.set_breakpoints(Path::new("src/main.rs"), &[]).is_ok());
}
```

# Errors

Returns [`DapError`] if communicating with the adapter fails.
