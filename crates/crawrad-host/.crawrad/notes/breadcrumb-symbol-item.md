---
id: breadcrumb-symbol-item
kind: doc
target_file: src/breadcrumb.rs
line_hint: 15
author: 
created: 
title: "symbol_item"
tags: []
signature: "pub fn symbol_item (text : & str , line : usize) -> Option < String >"
---

Returns the symbol enclosing the line for the breadcrumb.

The symbol the caret on `line` (0-based) of `text` is in, as the
breadcrumb's last item: `"<mark> name()"` for a fn, `"<mark> impl X"` for a
type, or `None` when nothing encloses it.

# Examples

```rust
use crawrad_host::breadcrumb::{symbol_item, METHOD};

let item = symbol_item("fn hello() {}", 0);
assert_eq!(item, Some(format!("{METHOD} hello()")));
```
