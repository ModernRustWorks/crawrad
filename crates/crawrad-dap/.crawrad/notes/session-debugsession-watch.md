---
id: session-debugsession-watch
kind: doc
target_file: src/session.rs
line_hint: 509
author: 
created: 
title: "DebugSession::watch"
tags: []
signature: "pub fn watch (& self , expression : & str , frame_id : i64 , presenter : & dyn ValuePresenter ,) -> crate :: ShownVariable"
---

Evaluates a watch expression in a frame and formats the result.

The text to show for `variable` in `frame_id`: the presenter's when it
knows the shape, the adapter's otherwise.
A watch: `expression` evaluated in `frame_id`, as the presenter shows
it — or, when the adapter cannot evaluate it, its own error as the
value (`<error: …>`), with nothing to open.

# Examples

```rust,no_run
// Evaluating a watch requires an active debug session.
use std::path::Path;
use crawrad_dap::{AdapterText, DebugSession, LaunchConfig};

if let Ok(session) = DebugSession::launch(
    Path::new("lldb-dap"),
    &LaunchConfig::default(),
    &std::collections::BTreeMap::<std::path::PathBuf, Vec<crawrad_dap::session::SourceBreakpoint>>::new(),
) {
    let shown = session.watch("x", 1, &AdapterText);
    assert_eq!(shown.name, "x");
}
```
