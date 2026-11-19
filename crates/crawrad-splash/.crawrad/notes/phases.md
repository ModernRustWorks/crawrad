---
id: phases
kind: doc
target_file: src/lib.rs
line_hint: 56
author: 
created: 
title: "PHASES"
tags: []
signature: "pub const PHASES : [& str ; 3] = [\"Starting the IDE\" , \"Preparing graphics\" , \"Showing the window\" ,]"
---

The steps, in the order the start goes through them.

# Examples

```rust
use crawrad_splash::PHASES;

assert_eq!(PHASES.len(), 3);
assert_eq!(PHASES[0], "Starting the IDE");
```
