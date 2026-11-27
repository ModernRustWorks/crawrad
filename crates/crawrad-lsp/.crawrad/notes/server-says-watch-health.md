---
id: server-says-watch-health
kind: doc
target_file: src/server_says.rs
line_hint: 85
author:
created:
title: "health"
tags: []
signature: "pub fn health (& self) -> Health"
---

The verdict as it stands.

A poisoned lock reads as `Fine`: a stuck watch must never invent a failure that
is not there.

# Examples

```rust
use crawrad_lsp::server_says::{Health, Watch};

let watch = Watch::new();
assert_eq!(watch.health(), Health::Fine);
```
