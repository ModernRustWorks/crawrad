---
id: targets-makepad-layout-visibility-expression
kind: doc
target_file: src/targets/makepad/layout.rs
line_hint: 57
author: 
created: 
title: "visibility_expression"
tags: []
signature: "pub fn visibility_expression (doc : & Document , reg : & crawrad_registry :: Registry , n : & Node ,) -> Result < String , crate :: Erro >"
---

Generates the runtime boolean visibility expression for a widget.

Generates the runtime boolean visibility expression for a widget,
taking into account its own `visible` flag, its parent containers (Panel/GroupBox),
and which tab of a `PageControl` it belongs to.

# Examples

```rust
use crawrad_codegen::targets::makepad::layout::visibility_expression;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let reg = Registry::builtin();
let doc = Document::new("form", "Form");
let node = Node::new("btn", "Button");
let expr = visibility_expression(&doc, &reg, &node).unwrap();
assert_eq!(expr, "self.form.btn.visible");
```

# Errors

Returns an error if any component or ancestor name is not a valid identifier.
