---
id: hover-diagnostics-at
kind: doc
target_file: src/hover.rs
line_hint: 131
author: 
created: 
title: "diagnostics_at"
tags: []
signature: "pub fn diagnostics_at (diagnostics : & [crate :: protocol :: Diagnostic] , at : Position) -> String"
---

Formats diagnostics intersecting position `at` for display in hover card.

The diagnostics whose range holds `at`, as the card writes them (#730):
`warning: ` and the message — what an attribute's author wrote kept,
the compiler's `on by default` footer left out — then `rustc · code`.
Hints are left out: they repeat a warning's help.

# Examples

```rust
use crawrad_lsp::hover::diagnostics_at;
use crawrad_lsp::protocol::Position;

let text = diagnostics_at(&[], Position::new(0, 0));
assert!(text.is_empty());
```
