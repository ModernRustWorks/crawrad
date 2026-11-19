---
id: generator-refused
kind: doc
target_file: src/generator.rs
line_hint: 89
author: 
created: 
title: "refused"
tags: []
signature: "pub fn refused (status : & str) -> bool"
---

Returns whether a status sentence indicates a refused generation.

Did the generation refuse, and is the sentence on the bar its explanation?

Pure, and therefore provable: the one asking is `Run`, which is only
measurable by opening the IDE.

# Examples

```rust
use crawrad_project::generator::refused;

assert!(refused("generate: error"));
assert!(!refused("generated for Makepad"));
```
