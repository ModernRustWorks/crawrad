---
id: options-numberchoice
kind: doc
target_file: src/options.rs
line_hint: 202
author: 
created: 
title: "NumberChoice"
tags: []
signature: "pub enum NumberChoice"
---

A number drop down of Options.

A number drop down of Options (File History & Backups, Autorecover): its
values, what it shows for each, the preference it reads and writes, and
its widget.

# Examples

```rust
use crawrad_project::options::NumberChoice;

assert_eq!(NumberChoice::ALL.len(), 4);
```
