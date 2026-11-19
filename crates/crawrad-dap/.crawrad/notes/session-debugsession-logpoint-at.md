---
id: session-debugsession-logpoint-at
kind: doc
target_file: src/session.rs
line_hint: 279
author: 
created: 
title: "DebugSession::logpoint_at"
tags: []
signature: "pub fn logpoint_at (& self , path : & Path , line : i64) -> Option < & str >"
---

The logpoint message at `line` of `path`, if a logpoint is there.

The adapter names the file the way the debug info spells it, which may
differ from the IDE's spelling in case or separators: the file on disk
decides, as `crawrad_host::breakpoints::file_key` does.

# Examples

```rust,no_run
// Querying logpoint messages requires an active debug session.
use std::path::Path;
use crawrad_dap::{DebugSession, LaunchConfig};

if let Ok(session) = DebugSession::launch(
    Path::new("lldb-dap"),
    &LaunchConfig::default(),
    &std::collections::BTreeMap::<std::path::PathBuf, Vec<crawrad_dap::session::SourceBreakpoint>>::new(),
) {
    assert_eq!(session.logpoint_at(Path::new("src/main.rs"), 1), None);
}
```
