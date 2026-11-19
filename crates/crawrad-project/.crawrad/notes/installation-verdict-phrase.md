---
id: installation-verdict-phrase
kind: doc
target_file: src/installation.rs
line_hint: 57
author: 
created: 
title: "Verdict::phrase"
tags: []
signature: "pub fn phrase (& self) -> String"
---

The phrase for the status bar — same for both outcomes.

# Examples

```rust
use crawrad_project::installation::Verdict;

let v = Verdict::Refused("missing file".to_string());
assert_eq!(v.phrase(), "Not installed: missing file");
```
