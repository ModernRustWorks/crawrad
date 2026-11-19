---
id: targets-egui-stubs-todos-os-eventos
kind: doc
target_file: src/targets/egui/stubs.rs
line_hint: 55
author: 
created: 
title: "todos_os_eventos"
tags: []
signature: "pub fn todos_os_eventos () -> Vec < & 'static str >"
---

Coleta dinamicamente todos os eventos cadastrados nos componentes do registro.

# Examples

```rust
use crawrad_codegen::targets::egui::stubs::todos_os_eventos;

let evs = todos_os_eventos();
assert!(evs.contains(&"on_click"));
```
