---
id: diff-review-hex-sha256
kind: doc
target_file: src/diff_review.rs
line_hint: 281
author: 
created: 
title: "hex_sha256"
tags: []
signature: "pub fn hex_sha256 (bytes : & [u8]) -> String"
---

Computes the SHA-256 hash of a byte sequence, as hexadecimal.

# Examples

```rust
use crawrad_agent::diff_review::hex_sha256;

let hash = hex_sha256(b"hello");
assert_eq!(hash.len(), 64);
```
