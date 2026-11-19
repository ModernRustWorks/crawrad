---
id: targets-egui-state-extra-inits
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 505
author: 
created: 
title: "EXTRA_INITS"
tags: []
signature: "pub const EXTRA_INITS : & [(& str , & str , & str)] = & [(\"Grid\" , \"cells\" , \"Default::default()\") , (\"Grid\" , \"selected\" , \"None\") , (\"Grid\" , \"editor\" , \"None\") , (\"MessageDialog\" , \"modal_result\" , \"ModalResult::None\") , (\"MessageDialog\" , \"answer\" , \"MessageAnswer::default()\") , (\"OpenDialog\" , \"files\" , \"Vec::new()\") , (\"Splitter\" , \"offset\" , \"0.0\") , (\"Timer\" , \"next_due\" , \"None\") , (\"TreeView\" , \"selection\" , \"Vec::new()\") ,]"
---

Initial values for target-specific extra fields that are not computed.

# Examples

```rust
use crawrad_codegen::targets::egui::state::EXTRA_INITS;

assert!(!EXTRA_INITS.is_empty());
```
