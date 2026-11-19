---
id: args-as-texts
kind: doc
target_file: src/lib.rs
line_hint: 246
author: 
created: 
title: "Args::as_texts"
tags: []
signature: "pub fn as_texts (& self , name : & str) -> Vec < & str >"
---

The list of texts of a parameter [`Param::list`].

Only reads `Value::List`, and inside it only `Str`: an element of another
type is an error of the caller, and creating something from it would be
inventing. Empty is what `execute` already knows how to handle — return
`Effect::none()`.

# Examples

```rust
use crawrad_actions::Args;

let args = Args::texts("names", vec!["A", "B"]);
assert_eq!(args.as_texts("names"), vec!["A", "B"]);
```
