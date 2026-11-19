---
id: options-numberchoice-position
kind: doc
target_file: src/options.rs
line_hint: 262
author: 
created: 
title: "NumberChoice::position"
tags: []
signature: "pub fn position (self , p : & crate :: workspace :: Preferences) -> Option < usize >"
---

Returns where the stored value sits in the list.

Where the stored value sits in the list (`None` when it was typed by
hand in the preferences file and is not offered).

# Examples

```rust
use crawrad_project::options::NumberChoice;
use crawrad_project::workspace::Preferences;

let p = Preferences::default();
assert!(NumberChoice::RecoverMinutes.position(&p).is_some());
```
