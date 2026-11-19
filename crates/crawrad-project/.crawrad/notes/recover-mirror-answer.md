---
id: recover-mirror-answer
kind: doc
target_file: src/recover.rs
line_hint: 79
author: 
created: 
title: "Mirror::answer"
tags: []
signature: "pub fn answer (& mut self , files : Vec < PathBuf > , discard_them : bool) -> Vec < (PathBuf , String) >"
---

Answers the recovery copies of `files`.

Answers the copies of `files` (none: every one waiting). Each copy
found comes back as its file and text, and stops waiting; a discarded
one is also deleted. A path that leaves the project finds nothing.

# Examples

```rust
use crawrad_project::recover::Mirror;

let mut mirror = Mirror::default();
let recovered = mirror.answer(Vec::new(), false);
assert!(recovered.is_empty());
```
