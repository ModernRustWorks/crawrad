---
id: targets-makepad-state-extra-fields
kind: doc
target_file: src/targets/makepad/state.rs
line_hint: 186
author: 
created: 
title: "EXTRA_FIELDS"
tags: []
signature: "pub const EXTRA_FIELDS : & [(& str , & str , & str)] = & [(\"DateTimePicker\" , \"shown\" , \"String\") , (\"PageControl\" , \"active_tab\" , \"usize\") , (\"Memo\" , \"lines\" , \"Strings\") , (\"TreeView\" , \"selected\" , \"Option<usize>\") , (\"TreeView\" , \"collapsed\" , \"std::collections::BTreeSet<usize>\") , (\"Splitter\" , \"offset\" , \"f32\") , (\"Splitter\" , \"press\" , \"f32\") , (\"MessageDialog\" , \"modal_result\" , \"ModalResult\") , (\"MessageDialog\" , \"answer\" , \"PendingAnswer\") , (\"OpenDialog\" , \"files\" , \"Vec<String>\") , (\"ListBox\" , \"selection\" , \"Vec<usize>\") , (\"Grid\" , \"cells\" , \"crawrad_runtime::string_grid::Cells\") , (\"Grid\" , \"selected\" , \"Option<(usize, usize)>\") ,]"
---

State that belongs to the target, and to no property at all.

**State that belongs to the TARGET, and to no property at all.**

A field here does not appear in the Object Inspector and the user never edits it: it
exists because the target needs to remember something between two frames.

It is the second valve, and it is different from [`FIELD_MAP`]: that one translates what
the catalog declares; this one adds what only the target knows it needs. Keeping
the two separate is what prevents someone from "solving" a missing property
by inventing state — the property field disappears from the inspector, and the
user never finds out why.

# Examples

```rust
use crawrad_codegen::targets::makepad::state::EXTRA_FIELDS;

assert!(EXTRA_FIELDS.iter().any(|(c, f, _)| *c == "PageControl" && *f == "active_tab"));
```
