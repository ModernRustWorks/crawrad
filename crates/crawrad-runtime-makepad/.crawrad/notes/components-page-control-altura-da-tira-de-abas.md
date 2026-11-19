---
id: components-page-control-altura-da-tira-de-abas
kind: doc
target_file: src/components/page_control.rs
line_hint: 18
author: 
created: 
title: "ALTURA_DA_TIRA_DE_ABAS"
tags: []
signature: "pub const ALTURA_DA_TIRA_DE_ABAS : f32 = 24.0"
---

Height of the tab strip, in design units.

**Mirrors `crawrad_registry::ALTURA_DA_TIRA_DE_ABAS`**, as the RXP already does
(`crawrad-runtime-egui/src/lib.rs:2852`): an adapter does not depend on the catalog.
The mirror is enforced by a test — see `canvas::tests::a_tira_de_abas_espelha_o_catalogo`.

# Examples

```rust
use crawrad_runtime_makepad::components::page_control::ALTURA_DA_TIRA_DE_ABAS;

assert_eq!(ALTURA_DA_TIRA_DE_ABAS, 24.0);
```
