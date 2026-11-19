---
id: inspector-model-column-answer
kind: doc
target_file: src/inspector_model.rs
line_hint: 162
author: 
created: 
title: "ColumnAnswer"
tags: []
signature: "pub type ColumnAnswer = Option < Result < Vec < String > , String > >"
---

A dataset's columns as the shell knows them: none yet, listed, or why not.

# Examples

```rust
use crawrad_actions::inspector_model::ColumnAnswer;

let missing: ColumnAnswer = Some(Err("crawrad-dbprobe not found next to the IDE".into()));
assert!(matches!(missing, Some(Err(_))));
```
