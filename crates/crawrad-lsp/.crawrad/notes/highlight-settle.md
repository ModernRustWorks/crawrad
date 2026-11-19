---
id: highlight-settle
kind: doc
target_file: src/highlight.rs
line_hint: 26
author: 
created: 
title: "SETTLE"
tags: []
signature: "pub const SETTLE : Duration = Duration :: from_millis (120)"
---

How long the caret must rest before highlighting occurrences.

How long the caret must rest before the server is asked. Shorter than the
inlay hints': moving the caret is a deliberate gesture, not typing.

# Examples

```rust
use crawrad_lsp::highlight::SETTLE;
use std::time::Duration;

assert_eq!(SETTLE, Duration::from_millis(120));
```
