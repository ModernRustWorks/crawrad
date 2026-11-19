---
id: contract-contracterror
kind: doc
target_file: src/contract.rs
line_hint: 213
author: 
created: 
title: "ContractError"
tags: []
signature: "pub enum ContractError"
---

What the ADR 0002 contract rejects.

**Read by the user**: the rejection bubbles up through `IrError::Contract` and the shell
writes it to the status bar. That is why it is in ENGLISH — see
`crawrad-doc/src/portao_ingles.rs`.

# Examples

```rust
use crawrad_ir::{ContractError, PropKey};

let err = PropKey::parse("unknown_target.prop").unwrap_err();
assert!(matches!(err, ContractError::UnknownTarget { .. }));
```
