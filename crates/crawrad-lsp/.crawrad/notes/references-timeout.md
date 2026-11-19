---
id: references-timeout
kind: doc
target_file: src/references.rs
line_hint: 19
author: 
created: 
title: "TIMEOUT"
tags: []
signature: "pub const TIMEOUT : Duration = Duration :: from_secs (10)"
---

How long references requests wait before timing out.

References can take longer than a definition: the server walks the
whole workspace.

# Examples

```rust
use crawrad_lsp::references::TIMEOUT;
use std::time::Duration;

assert_eq!(TIMEOUT, Duration::from_secs(10));
```
