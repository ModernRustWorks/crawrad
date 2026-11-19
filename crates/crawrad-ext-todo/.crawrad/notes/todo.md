---
id: todo
kind: doc
target_file: src/lib.rs
line_hint: 24
author: 
created: 
title: "Todo"
tags: []
signature: "pub struct Todo"
---

One tagged comment.

# Examples

```rust
use crawrad_ext_todo::Todo;

let todo = Todo {
    file: "src/main.rs".into(),
    line: 10,
    start: 4,
    end: 8,
    kind: "todo".into(),
    text: "clean up".into(),
};
assert_eq!(todo.line, 10);
```
