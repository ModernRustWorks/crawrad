---
id: session-sourcebreakpoint
kind: doc
target_file: src/session.rs
line_hint: 70
author: 
created: 
title: "SourceBreakpoint"
tags: []
signature: "pub struct SourceBreakpoint"
---

A breakpoint requested by the IDE, optionally with a log message.

A breakpoint as the IDE asks for it: a line and, for a LOGPOINT, the
message written when the line runs — the program does not stop.

The message is NOT handed to the adapter's `logMessage`: without Python,
LLDB writes a Rust value as its type name (`alloc::string::String`). The
IDE stops there, writes `{expr}` with its own presenter, and goes on — see
`controller::log_and_continue`.

# Examples

```rust
use crawrad_dap::SourceBreakpoint;

let bp = SourceBreakpoint::from(42);
assert_eq!(bp.line, 42);
assert!(bp.log_message.is_none());
```
