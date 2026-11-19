---
id: makepadctl
kind: doc
target_file: src/lib.rs
line_hint: 188
author: 
created: 
title: "MakepadCtl"
tags: []
signature: "pub struct MakepadCtl"
---

A component control in the Makepad runtime model.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_runtime::WindowControlBase;
use crawrad_runtime_makepad::MakepadCtl;
use std::collections::BTreeMap;

let doc = Document::new("Form1", "TForm");
let root_node = doc.node(&doc.root).unwrap();
let base = WindowControlBase::from_doc(&doc.root, root_node, &doc, true);
let ctl = MakepadCtl {
    base,
    props: BTreeMap::new(),
    itens: Vec::new(),
    item_symbols: Vec::new(),
    escolhido: None,
    marcado: false,
    pagina_ativa: 0,
    somente_leitura: false,
    intervalo: 0,
    borda: 0.0,
    scroll_y: 0.0,
    scroll_bars: String::new(),
    barra_h: false,
    barra_v: false,
    posicao_scroll: 0.0,
    desenho: crawrad_ir::drawing::Drawing::default(),
    design_rows: None,
    columns: Vec::new(),
    icon: Vec::new(),
    declared_class: String::new(),
};
assert_eq!(ctl.name, "Form1");
```
