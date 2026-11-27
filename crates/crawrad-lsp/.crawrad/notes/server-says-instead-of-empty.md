---
id: server-says-instead-of-empty
kind: doc
target_file: src/server_says.rs
line_hint: 65
author:
created:
title: "instead_of_empty"
tags: []
signature: "pub fn instead_of_empty (& self) -> Option < String >"
---

What to show the person instead of a count of zero.

`None` when an empty answer is the honest answer.

# Examples

```rust
use crawrad_lsp::server_says::Health;

assert!(Health::Fine.instead_of_empty().is_none());
let said = Health::ProjectFailed("failed to load manifest".into())
    .instead_of_empty()
    .unwrap();
assert!(said.contains("did not load"));
```
