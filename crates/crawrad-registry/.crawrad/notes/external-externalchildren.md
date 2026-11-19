---
id: external-externalchildren
kind: doc
target_file: src/external.rs
line_hint: 61
author: 
created: 
title: "ExternalChildren"
tags: []
signature: "pub enum ExternalChildren"
---

What a property IS, owned.

**The mirror of [`Children`], owning its text.**

Same reason as [`ExternalKind`]: the catalog stores `&'static str` and the
manifest arrives with `String`. The leak happens in a single place —
`into_catalogue` —, after validation has already rejected what it had to
reject.

# Examples

```rust
use crawrad_registry::external::ExternalChildren;

let children = ExternalChildren::None;
assert_eq!(children, ExternalChildren::default());
```
