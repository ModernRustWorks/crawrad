---
id: installation-verdict
kind: doc
target_file: src/installation.rs
line_hint: 43
author: 
created: 
title: "Verdict"
tags: []
signature: "pub enum Verdict"
---

Result of an installation attempt.

# Examples

```rust
use crawrad_project::installation::Verdict;

let v = Verdict::Refused("invalid manifest".to_string());
assert!(v.phrase().contains("Not installed"));
```
