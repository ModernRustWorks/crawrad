---
id: locate-serversource-status-name
kind: doc
target_file: src/locate.rs
line_hint: 49
author: 
created: 
title: "ServerSource::status_name"
tags: []
signature: "pub fn status_name (self) -> & 'static str"
---

Returns the server's descriptive name formatted for the status bar.

The server's name on the status bar. The machine's copy works, but it
is not the version the IDE was tested with, and the bar says so.

# Examples

```rust
use crawrad_lsp::locate::ServerSource;

assert_eq!(ServerSource::System.status_name(), "rust-analyzer (system copy)");
```
