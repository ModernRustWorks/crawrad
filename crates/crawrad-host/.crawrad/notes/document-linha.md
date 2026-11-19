---
id: document-linha
kind: doc
target_file: src/document.rs
line_hint: 197
author: 
created: 
title: "Linha"
tags: []
signature: "pub struct Linha"
---

A line of the structure tree.

The name and class come **separated**, and not in a single string, because the screen
paints them in different colors: the name in the text color and the class in the tertiary color,
which is what the owner's screenshot shows. If concatenated here, the only way to
separate them again would be to search for the separator inside the text — and a component
name can contain anything.

# Examples

```rust
use crawrad_host::document::Linha;

let linha = Linha { id: "btn".into(), nivel: 0, nome: "Button1".into(), classe: "Button".into() };
assert_eq!(linha.sufixo_de_classe(), "  ·  Button");
```
