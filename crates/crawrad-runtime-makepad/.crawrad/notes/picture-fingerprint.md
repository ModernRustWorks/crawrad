---
id: picture-fingerprint
kind: doc
target_file: src/picture.rs
line_hint: 40
author: 
created: 
title: "fingerprint"
tags: []
signature: "pub fn fingerprint (bytes : & [u8]) -> u64"
---

Computes a fast hash fingerprint from image byte samples.

A cheap fingerprint: the length and up to 64 bytes spread over the data —
enough to tell one picture from the next without hashing megabytes on
every mouse move.

# Examples

```rust
use crawrad_runtime_makepad::picture::fingerprint;

let fp = fingerprint(b"sample image bytes");
assert_ne!(fp, 0);
```
