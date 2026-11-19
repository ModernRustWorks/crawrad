---
id: references-before
kind: doc
target_file: src/references.rs
line_hint: 84
author: 
created: 
title: "BEFORE"
tags: []
signature: "pub const BEFORE : usize = 16"
---

Maximum character count shown before a referenced name in preview rows.

How much of the line shows before the name (little: in a narrow panel the
name must stay in sight, as a modern editor keeps it), and after it.

# Examples

```rust
use crawrad_host::references::BEFORE;

assert_eq!(BEFORE, 16);
```
