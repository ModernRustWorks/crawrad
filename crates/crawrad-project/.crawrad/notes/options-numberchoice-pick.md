---
id: options-numberchoice-pick
kind: doc
target_file: src/options.rs
line_hint: 273
author: 
created: 
title: "NumberChoice::pick"
tags: []
signature: "pub fn pick (self , p : & mut crate :: workspace :: Preferences , pos : usize)"
---

Stores the value picked at `pos`; an index out of the list changes nothing.

# Examples

```rust
use crawrad_project::options::NumberChoice;
use crawrad_project::workspace::Preferences;

let mut p = Preferences::default();
NumberChoice::RecoverMinutes.pick(&mut p, 0);
assert_eq!(p.recover_minutes, 1);
```
