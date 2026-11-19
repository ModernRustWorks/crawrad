---
id: debugsession
kind: doc
target_file: src/lib.rs
line_hint: 1
author: 
created: 
title: "DebugSession"
tags: []
signature: "pub enum DebugSession"
---

Where the debugger is, as the action catalogue needs it.

`None` while nothing is being debugged, `Running` while the program runs
under the debugger, `Stopped` at a breakpoint or after a step. Step Into,
Over and Out need `Stopped`, Pause needs `Running`, Stop needs either: with
no session they are greyed instead of answering *"no program is being
debugged"* after the click.

# Examples

```rust
use crawrad_actions::{DebugSession, Project};

assert_eq!(Project::default().debug, DebugSession::None);
```
