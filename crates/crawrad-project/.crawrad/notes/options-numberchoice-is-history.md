---
id: options-numberchoice-is-history
kind: doc
target_file: src/options.rs
line_hint: 234
author: 
created: 
title: "NumberChoice::is_history"
tags: []
signature: "pub fn is_history (self) -> bool"
---

Whether it sits on the File History & Backups page.

# Examples

```rust
use crawrad_project::options::NumberChoice;

assert!(NumberChoice::Limit.is_history());
assert!(!NumberChoice::RecoverMinutes.is_history());
```
