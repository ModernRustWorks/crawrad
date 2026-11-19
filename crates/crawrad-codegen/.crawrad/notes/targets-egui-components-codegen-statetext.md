---
id: targets-egui-components-codegen-statetext
kind: doc
target_file: src/targets/egui/components/codegen.rs
line_hint: 41
author: 
created: 
title: "StateText"
tags: []
signature: "pub struct StateText"
---

What a class writes around its state struct in the `.designer.rs`.

# Examples

```rust
use crawrad_codegen::targets::egui::components::codegen::StateText;

let st = StateText { head: "struct S {", tail: "}" };
assert_eq!(st.head, "struct S {");
```
