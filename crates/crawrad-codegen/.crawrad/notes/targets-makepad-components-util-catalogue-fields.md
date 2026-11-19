---
id: targets-makepad-components-util-catalogue-fields
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 172
author: 
created: 
title: "catalogue_fields"
tags: []
signature: "pub fn catalogue_fields (desc : & ComponentDesc , by_hand : & [& str]) -> Vec < String >"
---

Returns catalogue properties that a state struct carries as plain fields.

The catalogue properties a hand-written state struct carries as plain fields: every property of `desc`
that is emitted (`why_not_set` is `None`) and that the component does not write by hand (`by_hand`).
`name`, `rect` and `action` never become fields, so callers list only their own hand-written keys.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::catalogue_fields;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let desc = reg.get("Button").unwrap();
let fields = catalogue_fields(desc, &["text"]);
assert!(!fields.is_empty());
```
