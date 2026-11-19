---
id: definition-retrying
kind: doc
target_file: src/definition.rs
line_hint: 129
author: 
created: 
title: "retrying"
tags: []
signature: "pub fn retrying < T > (request : & mut dyn FnMut (Duration) -> Result < T , crate :: LspError > , timeout : Duration , retry : Duration ,) -> Result < T , crate :: LspError >"
---

Retries a request while the server reports content modification.

Asks `request` (it gets the time left) again while the server answers
[`CONTENT_MODIFIED`] and `timeout` allows; any other answer is final.
Shared by every worker that asks about a position (definition, hover).

# Examples

```rust
use crawrad_lsp::definition::retrying;
use std::time::Duration;

let mut req = |_left| Ok(42);
let val = retrying(&mut req, Duration::from_secs(1), Duration::from_millis(10)).unwrap();
assert_eq!(val, 42);
```

# Errors

Returns any non-retryable error produced by the request callback.
