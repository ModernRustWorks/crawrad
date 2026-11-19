---
id: args-text
kind: doc
target_file: src/lib.rs
line_hint: 215
author: 
created: 
title: "Args::text"
tags: []
signature: "pub fn text (name : impl Into < String > , text : impl Into < String >) -> Self"
---

The common case: a single text, under the name the action publishes.

It exists so the caller does not write `Value::Str(x.into())` on every
line — and **only** for that. Whoever sends a number, boolean or list goes
through [`Args::one`] with the `Value` in hand, because widening the type for
convenience was exactly what made `Vec<String>` become the only
possible form.

# Examples

```rust
use crawrad_actions::Args;

let args = Args::text("name", "Button1");
assert_eq!(args.as_text("name"), Some("Button1"));
```
