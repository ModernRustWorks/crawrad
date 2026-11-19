---
id: link-localizar-handler
kind: doc
target_file: src/link.rs
line_hint: 54
author: 
created: 
title: "localizar_handler"
tags: []
signature: "pub fn localizar_handler (fonte : & str , nome : & str) -> Option < Local >"
---

Where the `fn` named `nome` is **defined**, if it exists.

# Examples

```rust
use crawrad_codegen::link::localizar_handler;

let src = "impl Form { fn on_click(&mut self) {} }";
let loc = localizar_handler(src, "on_click");
assert!(loc.is_some());
```

Returns the **first** definition. Two definitions with the same name do not
compile in the same `impl`, but compile in different `impl`s (traits, or
an `impl` of another type in the same file) — and choosing the first is then a
choice, not a certainty. Whoever needs to know that a tie occurred queries
[`handlers_do_arquivo`], which lists both.
