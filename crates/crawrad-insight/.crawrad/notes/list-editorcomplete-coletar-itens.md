---
id: list-editorcomplete-coletar-itens
kind: doc
target_file: src/list.rs
line_hint: 79
author: 
created: 
title: "EditorComplete::coletar_itens"
tags: []
signature: "pub fn coletar_itens (prefixo : & str , texto : & str , base : Option < & str >) -> Vec < ItemSugestao >"
---

The local list — see [`crate::item::local_items`].

# Examples

```rust
use crawrad_insight::list::EditorComplete;

let items = EditorComplete::coletar_itens("le", "let x = 1;", None);
assert!(items.iter().any(|i| i.rotulo == "let"));
```
