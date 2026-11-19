---
id: targets-egui-state-extra-fields
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 257
author: 
created: 
title: "EXTRA_FIELDS"
tags: []
signature: "pub const EXTRA_FIELDS : & [(& str , & str , & str)] = & [(\"DateTimePicker\" , \"shown\" , \"String\") , (\"Grid\" , \"cells\" , \"crawrad_runtime::string_grid::Cells\") , (\"Grid\" , \"selected\" , \"Option<(usize, usize)>\") , (\"Grid\" , \"editor\" , \"Option<String>\") , (\"ListBox\" , \"selection\" , \"Vec<usize>\") , (\"Memo\" , \"lines\" , \"Strings\") , (\"MessageDialog\" , \"modal_result\" , \"ModalResult\") , (\"MessageDialog\" , \"answer\" , \"MessageAnswer\") , (\"OpenDialog\" , \"files\" , \"Vec<String>\") , (\"Splitter\" , \"offset\" , \"f32\") , (\"Timer\" , \"next_due\" , \"Option<f64>\") , (\"TreeView\" , \"selection\" , \"Vec<usize>\") , (\"TreeView\" , \"collapsed\" , \"Vec<Vec<usize>>\") ,]"
---

Target-specific fields not mapped from catalogue properties.

# Examples

```rust
use crawrad_codegen::targets::egui::state::EXTRA_FIELDS;

assert!(!EXTRA_FIELDS.is_empty());
```
