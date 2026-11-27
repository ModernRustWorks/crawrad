---
id: server-says-cannot-trust-empty
kind: doc
target_file: src/server_says.rs
line_hint: 57
author:
created:
title: "cannot_trust_empty"
tags: []
signature: "pub fn cannot_trust_empty (& self) -> bool"
---

True while an empty answer cannot be trusted to mean "there are none".

Indexing is the other such moment, and the client answers for it with
`LspClient::progress`.

# Examples

```rust
use crawrad_lsp::server_says::Health;

assert!(Health::ProjectFailed("no manifest".into()).cannot_trust_empty());
assert!(!Health::Fine.cannot_trust_empty());
```
