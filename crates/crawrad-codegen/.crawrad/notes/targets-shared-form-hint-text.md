---
id: targets-shared-form-hint-text
kind: doc
target_file: src/targets/shared.rs
line_hint: 51
author: 
created: 
title: "form_hint_text"
tags: []
signature: "pub fn form_hint_text (root : & crawrad_ir :: Node) -> Option < & str >"
---

Returns the form's hint text when enabled and non-empty.

The form's `Hint`, when it has one to show (`ShowHint` and a hint that is
not empty). The form keeps no state struct (#462), so it is the
designer's value.

# Examples

```rust
use crawrad_codegen::targets::shared::form_hint_text;
use crawrad_ir::Node;

let root = Node::new("form1", "Form");
assert_eq!(form_hint_text(&root), None);
```
