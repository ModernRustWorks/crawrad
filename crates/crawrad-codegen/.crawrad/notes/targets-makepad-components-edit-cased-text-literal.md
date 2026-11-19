---
id: targets-makepad-components-edit-cased-text-literal
kind: doc
target_file: src/targets/makepad/components/edit.rs
line_hint: 85
author: 
created: 
title: "cased_text_literal"
tags: []
signature: "pub fn cased_text_literal (node : & Node) -> String"
---

The initial text literal of an Edit box held to its CharCase.

The text a box of the `Edit` family opens with, as a literal: held to its
`CharCase` (#667).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::edit::cased_text_literal;
use crawrad_ir::Node;

let node = Node::new("ed", "Edit");
let lit = cased_text_literal(&node);
assert_eq!(lit, "\"ed\"");
```
