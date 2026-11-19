---
id: inspector-model-editors-reference-reference-new
kind: doc
target_file: src/inspector_model/editors/reference.rs
line_hint: 42
author: 
created: 
title: "Reference::new"
tags: []
signature: "pub fn new (pairs : Vec < (String , String) > , value : & str) -> Self"
---

`pairs` is `(id, name)` for every candidate, in the order they appear.

# Examples

```rust
use crawrad_actions::inspector_model::editors::reference::Reference;

let r = Reference::new(vec![("act1".to_string(), "Action 1".to_string())], "act1");
assert_eq!(format!("{r:?}"), "Reference { labels: [\"(none)\", \"Action 1\"], ids: [\"\", \"act1\"], current: Some(1) }");
```
