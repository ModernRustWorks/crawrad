---
id: session-debugsession-pump
kind: doc
target_file: src/session.rs
line_hint: 291
author: 
created: 
title: "DebugSession::pump"
tags: []
signature: "pub fn pump (& mut self , timeout : Duration) -> Result < Option < Event > , DapError >"
---

Takes one event, if one comes within `timeout`, and applies it.

# Examples

```rust,no_run
// Pumping events requires an active debug session.
use std::path::Path;
use std::time::Duration;
use crawrad_dap::{DebugSession, LaunchConfig};

if let Ok(mut session) = DebugSession::launch(
    Path::new("lldb-dap"),
    &LaunchConfig::default(),
    &std::collections::BTreeMap::<std::path::PathBuf, Vec<crawrad_dap::session::SourceBreakpoint>>::new(),
) {
    let _ = session.pump(Duration::from_millis(10));
}
```

# Errors

Returns [`DapError::Closed`] if the connection ended, or a transport error.
