---
id: targets-makepad-components-combobox-emit-sync
kind: doc
target_file: src/targets/makepad/components/combobox.rs
line_hint: 110
author: 
created: 
title: "emit_sync"
tags: []
signature: "pub fn emit_sync (_doc : & Document , _node : & Node , field : & str) -> String"
---

Synchronizes the state's `selected` with the on-screen component.

**The state's `selected` reaches the screen**, whereas before it did not.

This function used to return `String::new()`: the user wrote
`self.combo_box1.selected = Some(2);` in their file, ran it, and the box
continued showing what was there before. The field existed, the assignment
compiled, and nothing happened — the same family as issue #141, the whole
path working except the last step.

# The framework's limit, declared

Returning to **"nothing selected"** at runtime is not possible:
`DropDownRef::set_selected_item` clamps the index to the last item in the list
(`makepad-widgets-1.0.0/src/drop_down.rs:960`), so a `None` coming from
user code cannot turn into an empty box. The INITIAL `None`
works, because it goes through the DSL's `selected_item`, which does not go through
that clamping — and that is why `emit_dsl` and this `emit_sync` say the same
thing through different paths.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::combobox::emit_sync;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("cb", "ComboBox");
let sync = emit_sync(&doc, &node, "cb");
assert!(sync.contains("cb"));
```
