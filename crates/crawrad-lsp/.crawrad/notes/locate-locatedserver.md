---
id: locate-locatedserver
kind: doc
target_file: src/locate.rs
line_hint: 58
author: 
created: 
title: "LocatedServer"
tags: []
signature: "pub struct LocatedServer"
---

A server found, and where.

# Examples

```rust
use crawrad_lsp::locate::{LocatedServer, ServerSource};
use std::path::PathBuf;

let server = LocatedServer { path: PathBuf::from("rust-analyzer"), source: ServerSource::Bundled };
assert_eq!(server.source, ServerSource::Bundled);
```
