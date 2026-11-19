---
id: debug-panel-pretty-function
kind: doc
target_file: src/debug_panel.rs
line_hint: 253
author: 
created: 
title: "pretty_function"
tags: []
signature: "pub fn pretty_function (name : & str) -> String"
---

A frame's function as a person reads it.

The adapter gives the linker's name for trait methods, measured on
30/09/2026: `_$LT$meu_app..form_main_designer..App$u20$as$u20$makepad_draw..match_event..MatchEvent$GT$::handle_actions::ha5155190f5088096`.
This undoes the escapes of that mangling (`$LT$` `<`, `$GT$` `>`, `$u20$`
space, `$C$` comma, `..` `::`), drops the `::h…` hash and the leading `_`,
and leaves an already readable name as it is.

# Examples

```rust
use crawrad_host::debug_panel::pretty_function;

assert_eq!(pretty_function("my_crate::my_fn"), "my_crate::my_fn");
```
