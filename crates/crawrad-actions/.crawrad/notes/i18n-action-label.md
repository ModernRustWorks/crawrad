---
id: i18n-action-label
kind: doc
target_file: src/i18n.rs
line_hint: 34
author: 
created: 
title: "action_label"
tags: []
signature: "pub fn action_label (action_id : & str , default_label : & 'static str , lang : Language) -> & 'static str"
---

Translates an action label given its Action ID and default English label.

# Examples

```rust
use crawrad_actions::i18n::{action_label, Language};

let label = action_label("file.save", "Save", Language::Portuguese);
assert_eq!(label, "Salvar");
```
