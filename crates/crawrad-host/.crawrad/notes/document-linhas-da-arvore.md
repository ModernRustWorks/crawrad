---
id: document-linhas-da-arvore
kind: doc
target_file: src/document.rs
line_hint: 219
author: 
created: 
title: "linhas_da_arvore"
tags: []
signature: "pub fn linhas_da_arvore (doc : & Document) -> Vec < Linha >"
---

Returns all lines of the document tree in display order.

The entire tree in lines, in the order it is read: the root, then each
child with its descendants right below it.

The order is that of the document, which is the Z-order within the parent — the same one
that `Document::em_ordem_de_desenho` uses. The recursion descends into EVERYTHING, including
the `ActionList`, whose actions are its children (it is the ActionList editor of a
classic RAD IDE, without the extra window).

# Examples

```rust
use crawrad_ir::Document;
use crawrad_host::document::linhas_da_arvore;

let doc = Document::new("form", "Form");
let linhas = linhas_da_arvore(&doc);
assert_eq!(linhas.len(), 1);
```
