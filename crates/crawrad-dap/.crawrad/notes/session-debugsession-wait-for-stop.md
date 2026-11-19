---
id: session-debugsession-wait-for-stop
kind: doc
target_file: src/session.rs
line_hint: 346
author: 
created: 
title: "DebugSession::wait_for_stop"
tags: []
signature: "pub fn wait_for_stop (& mut self , timeout : Duration) -> Result < & SessionState , DapError >"
---

Waits until the program stops or ends, and says which.

# A step lands on the next line, not on the same one again

A breakpoint on a line whose code is split in several places (a
`println!` expands into several) is bound to all of them. Measured on
30/09/2026 with `lldb-dap`: a `next` from that line stopped on the same
line with `reason: breakpoint`, twice, before reaching the next one.
F8 moves to the next line, so when a step stops at a breakpoint on the
very line and function it left from, the step is issued again — up to
`MAX_STEP_RETRIES` times, never forever.

The same line hits a Continue too (measured on 30/09/2026: F9 from a
breakpoint on `format!(…)` in a click handler stopped on that line
again). There the instruction tells them apart: another address of the
same line is continued past; the very same address is a loop coming
round, and stops.

# Examples

```rust,no_run
// Waiting for a stop requires an active debug session.
use std::path::Path;
use std::time::Duration;
use crawrad_dap::{DebugSession, LaunchConfig};

if let Ok(mut session) = DebugSession::launch(
    Path::new("lldb-dap"),
    &LaunchConfig::default(),
    &std::collections::BTreeMap::<std::path::PathBuf, Vec<crawrad_dap::session::SourceBreakpoint>>::new(),
) {
    let _ = session.wait_for_stop(Duration::from_millis(50));
}
```

# Errors

Returns [`DapError`] on timeout or if the adapter connection drops.
