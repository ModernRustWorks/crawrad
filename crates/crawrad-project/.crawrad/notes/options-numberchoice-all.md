---
id: options-numberchoice-all
kind: doc
target_file: src/options.rs
line_hint: 216
author: 
created: 
title: "NumberChoice::ALL"
tags: []
signature: "pub const ALL : [Self ; 4] = [Self :: Limit , Self :: MergeSeconds , Self :: MaxFileKb , Self :: RecoverMinutes ,]"
---

The four, in the order the pages show them.

# Examples

```rust
use crawrad_project::options::NumberChoice;

assert_eq!(NumberChoice::ALL[0], NumberChoice::Limit);
```
