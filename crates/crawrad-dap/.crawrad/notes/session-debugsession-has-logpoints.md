---
id: session-debugsession-has-logpoints
kind: doc
target_file: src/session.rs
line_hint: 274
author: 
created: 
title: "DebugSession::has_logpoints"
tags: []
signature: "pub fn has_logpoints (& self) -> bool"
---

Returns whether any logpoint is currently configured.

Whether any logpoint is set: without one, a stop is never a logpoint's
and costs no extra request.

# Examples

```rust,no_run
// Querying logpoints requires an active debug session.
use std::path::Path;
use crawrad_dap::{DebugSession, LaunchConfig};

if let Ok(session) = DebugSession::launch(
    Path::new("lldb-dap"),
    &LaunchConfig::default(),
    &std::collections::BTreeMap::<std::path::PathBuf, Vec<crawrad_dap::session::SourceBreakpoint>>::new(),
) {
    assert!(!session.has_logpoints());
}
```
