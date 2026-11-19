---
id: list-editorcomplete-coletar-sugestoes
kind: doc
target_file: src/list.rs
line_hint: 84
author: 
created: 
title: "EditorComplete::coletar_sugestoes"
tags: []
signature: "pub fn coletar_sugestoes (prefixo : & str , texto_documento : & str) -> Vec < String >"
---

Preserves compatibility with existing test suite signatures.

Preserves compatibility with the existing test signature.

# Examples

```rust
use crawrad_insight::list::EditorComplete;

let sugestoes = EditorComplete::coletar_sugestoes("le", "let x = 1;");
assert!(sugestoes.contains(&"let".to_string()));
```
