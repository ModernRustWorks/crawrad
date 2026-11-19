---
id: references-hit
kind: doc
target_file: src/references.rs
line_hint: 18
author: 
created: 
title: "Hit"
tags: []
signature: "pub struct Hit"
---

One occurrence: the line (0-based), the symbol's bytes in it, the line.

# Examples

```rust
use crawrad_host::references::Hit;

let hit = Hit::new(0, 4, 7, "let var = 1;".into());
assert_eq!(hit.line, 0);
assert_eq!(hit.start, 4);
```
