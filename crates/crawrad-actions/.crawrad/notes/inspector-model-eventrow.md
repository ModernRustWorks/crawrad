---
id: inspector-model-eventrow
kind: doc
target_file: src/inspector_model.rs
line_hint: 240
author: 
created: 
title: "EventRow"
tags: []
signature: "pub struct EventRow"
---

An event row shown on screen.

An event on screen: the name from the catalogue, the label and the bound
handler.

**No event name is written in this shell.** The three fields come from the
`crawrad_registry::EventDesc` and from the document; an event new in the
catalogue shows up here without a line edited — the same rule as the
properties, and what
`todo_evento_do_catalogo_vira_linha_sem_lista_a_mao` charges.

# Examples

```rust
use crawrad_actions::inspector_model::EventRow;

let row = EventRow {
    event: "on_click",
    label: "OnClick",
    handler: "Button1Click".to_string(),
    signature: "fn()",
    base_handler: None,
    base_form: None,
};
assert_eq!(row.event, "on_click");
```
