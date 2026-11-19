---
id: document-linha-sufixo-de-classe
kind: doc
target_file: src/document.rs
line_hint: 210
author: 
created: 
title: "Linha::sufixo_de_classe"
tags: []
signature: "pub fn sufixo_de_classe (& self) -> String"
---

Suffix with separator and class displayed after the name in the tree.

The suffix that the screen draws after the name — the separator and class
joined together, because both appear and disappear together.

# Examples

```rust
use crawrad_host::document::Linha;

let linha = Linha { id: "f".into(), nivel: 0, nome: "Form".into(), classe: "".into() };
assert_eq!(linha.sufixo_de_classe(), "");
```
