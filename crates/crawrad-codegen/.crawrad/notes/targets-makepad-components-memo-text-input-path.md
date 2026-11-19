---
id: targets-makepad-components-memo-text-input-path
kind: doc
target_file: src/targets/makepad/components/memo.rs
line_hint: 208
author: 
created: 
title: "text_input_path"
tags: []
signature: "pub fn text_input_path (node : & Node) -> String"
---

The path used to address this Memo's TextInput widget.

The path used to address this `Memo`'s `TextInput` widget:
`{name}` when `scroll_bars` is `ssNone`, `{name}.{name}_text` otherwise.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::memo::text_input_path;
use crawrad_ir::Node;

let node = Node::new("memo", "Memo");
assert_eq!(text_input_path(&node), "memo");
```
