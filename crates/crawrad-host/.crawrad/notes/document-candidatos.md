---
id: document-candidatos
kind: doc
target_file: src/document.rs
line_hint: 24
author: 
created: 
title: "CANDIDATOS"
tags: []
signature: "pub const CANDIDATOS : & [& str] = & [\"examples/calculator/calculator/src/form_main.crawform\" , \"examples/contacts/contacts/src/form_main.crawform\" , \"examples/contacts-edit/contacts_edit/src/form_main.crawform\" ,]"
---

Default repository forms in priority order.

The REPOSITORY forms, in order: the default when nobody passes
any path. It is the same list as in example
`crawrad-runtime-makepad/examples/design_canvas.rs`.

# Examples

```rust
use crawrad_host::document::CANDIDATOS;

assert!(!CANDIDATOS.is_empty());
```
