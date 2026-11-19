---
id: client-cancel-poll
kind: doc
target_file: src/client.rs
line_hint: 119
author: 
created: 
title: "CANCEL_POLL"
tags: []
signature: "pub const CANCEL_POLL : Duration = Duration :: from_millis (10)"
---

How often a waiting request asks whether a newer one replaced it.

# Examples

```rust
use crawrad_lsp::client::CANCEL_POLL;
use std::time::Duration;

assert_eq!(CANCEL_POLL, Duration::from_millis(10));
```
