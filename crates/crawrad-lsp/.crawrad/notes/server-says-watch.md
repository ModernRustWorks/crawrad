---
id: server-says-watch
kind: doc
target_file: src/server_says.rs
line_hint: 75
author:
created:
title: "Watch"
tags: []
signature: "pub struct Watch (Arc < Mutex < Health >>)"
---

The health the client keeps, shared with whoever asks.

# Examples

```rust
use crawrad_lsp::server_says::{Health, Watch};

let watch = Watch::new();
assert_eq!(watch.health(), Health::Fine);
```
