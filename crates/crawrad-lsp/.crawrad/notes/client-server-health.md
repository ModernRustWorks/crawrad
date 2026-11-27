---
id: client-server-health
kind: doc
target_file: src/client.rs
line_hint: 680
author:
created:
title: "server_health"
tags: []
signature: "pub fn server_health (& self) -> crate :: server_says :: Health"
---

What the server last said about its own health.

The project failing to load, above all. Read it when an answer comes back EMPTY:
that is the one moment where "there are none" and "the server knows nothing" look
exactly the same.

# Examples

```rust
// A client that has heard nothing reports a healthy server.
use crawrad_lsp::server_says::Health;

assert_eq!(Health::default(), Health::Fine);
```
