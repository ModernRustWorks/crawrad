---
id: form-json-component
kind: doc
target_file: src/form_json.rs
line_hint: 107
author: 
created: 
title: "component"
tags: []
signature: "pub fn component (doc : & crawrad_doc :: Doc , id : & str) -> Option < (Value , String) >"
---

Returns the node by id or name with its properties, events and bindings.

`designer.get_component_properties`: the node by id, else by name, with
its properties, events and data bindings; `None` when there is none.

The bindings ride along because a bound property's value in `properties`
is NOT what the app shows — the dataset is (#788). Without them an agent
read `text: "edit1"` and had no way to tell the Inspector said `from dst1.name`.

# Examples

```rust
use crawrad_doc::Doc;
use crawrad_ir::Document;
use crawrad_mcp::form_json::component;

let doc = Doc::new(Document::new("form_main", "Form"));
let result = component(&doc, "form_main");
assert!(result.is_some());
assert_eq!(result.unwrap().1, "form_main");
```
