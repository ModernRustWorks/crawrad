---
id: args-texts
kind: doc
target_file: src/lib.rs
line_hint: 220
author: 
created: 
title: "Args::texts"
tags: []
signature: "pub fn texts < I , S > (name : impl Into < String > , items : I) -> Self where I : IntoIterator < Item = S > , S : Into < String > ,"
---

Several texts under one name — the parameter with [`Param::list`].

# Examples

```rust
use crawrad_actions::Args;

let args = Args::texts("items", vec!["first", "second"]);
assert_eq!(args.as_texts("items"), vec!["first", "second"]);
```
