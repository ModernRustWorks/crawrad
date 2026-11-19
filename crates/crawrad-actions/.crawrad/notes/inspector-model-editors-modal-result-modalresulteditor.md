---
id: inspector-model-editors-modal-result-modalresulteditor
kind: doc
target_file: src/inspector_model/editors/modal_result.rs
line_hint: 18
author: 
created: 
title: "ModalResultEditor"
tags: []
signature: "pub struct ModalResultEditor"
---

A drop-down over modal result options.

# Examples

```rust
use crawrad_actions::inspector_model::editors::modal_result::ModalResultEditor;
use crawrad_actions::inspector_model::editor::PropEditor;
use crawrad_ir::ModalResult;

let ed = ModalResultEditor::new(ModalResult::Ok);
assert_eq!(ed.template(), "RowChoice");
```
