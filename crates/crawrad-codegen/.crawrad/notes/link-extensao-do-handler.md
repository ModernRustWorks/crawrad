---
id: link-extensao-do-handler
kind: doc
target_file: src/link.rs
line_hint: 110
author: 
created: 
title: "extensao_do_handler"
tags: []
signature: "pub fn extensao_do_handler (fonte : & str , byte : usize) -> Option < Extensao >"
---

Measures the `fn` whose **name** starts at `byte` (from [`Local`]).

# Examples

```rust
use crawrad_codegen::link::{extensao_do_handler, localizar_handler};

let src = "fn test() { let x = 1; }";
let loc = localizar_handler(src, "test").unwrap();
let ext = extensao_do_handler(src, loc.byte);
assert!(ext.is_some());
```

Braces are counted in the **masked** source: a `{` inside
`println!("{}")` or inside a comment opens nothing, and counting it would make the
slice stop in the middle of the body — or never stop at all.

`None` when the body does not close (half-written file while typing).
In that case nothing is touched: measuring wrong is worse than not measuring.
