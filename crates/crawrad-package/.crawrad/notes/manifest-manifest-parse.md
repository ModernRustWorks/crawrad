---
id: manifest-manifest-parse
kind: doc
target_file: src/manifest.rs
line_hint: 823
author: 
created: 
title: "Manifest::parse"
tags: []
signature: "pub fn parse (text : & str) -> Result < Self , ManifestError >"
---

Parses a manifest from JSON text, or returns an error.

**Reads a manifest, or says why it cannot.**

The SDK version is checked **before** anything else: a
file of a contract this IDE does not speak could have fields that it
would interpret incorrectly, and the refusal must be about the contract and not about
the first strange field.

# Examples

```rust
use crawrad_package::Manifest;

let res = Manifest::parse("{}");
assert!(res.is_err());
```

# Errors

Returns [`ManifestError`](crate::ManifestError) if JSON parsing fails, the SDK version is incompatible, or fields are invalid.
