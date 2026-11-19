---
id: session-request-timeout
kind: doc
target_file: src/session.rs
line_hint: 31
author: 
created: 
title: "REQUEST_TIMEOUT"
tags: []
signature: "pub const REQUEST_TIMEOUT : Duration = Duration :: from_secs (30)"
---

How long one request may take.

# Examples

```rust
use crawrad_dap::session::REQUEST_TIMEOUT;

assert!(REQUEST_TIMEOUT.as_secs() > 0);
```
