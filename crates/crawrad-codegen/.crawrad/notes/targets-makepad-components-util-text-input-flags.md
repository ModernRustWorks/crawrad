---
id: targets-makepad-components-util-text-input-flags
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 122
author: 
created: 
title: "text_input_flags"
tags: []
signature: "pub fn text_input_flags (node : & Node) -> String"
---

Emits `TextInput` DSL lines for password and read-only flags.

A `TextInput`'s `is_password` / `is_read_only` lines (#434): a field with a
`password_char` never shows what is typed, a `readonly` one cannot be edited.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::text_input_flags;
use crawrad_ir::Node;

let node = Node::new("edit1", "Edit");
let flags = text_input_flags(&node);
assert!(flags.is_empty());
```
