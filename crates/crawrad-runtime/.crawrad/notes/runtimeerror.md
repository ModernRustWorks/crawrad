---
id: runtimeerror
kind: doc
target_file: src/lib.rs
line_hint: 146
author: 
created: 
title: "RuntimeError"
tags: []
signature: "pub enum RuntimeError"
---

What the adapter rejects.

**Read by the user**: the shell writes `runtime: <message>` to the status
bar (`crawrad-ide/src/main.rs:1848` and `:3941`). That is why it is in ENGLISH —
enforced by `crawrad-doc/src/portao_ingles.rs`.

# Examples

```rust
use crawrad_runtime::RuntimeError;

let err = RuntimeError::Unsupported("TUnknown".into());
assert_eq!(err.to_string(), "component 'TUnknown' is not supported by this runtime");
```
