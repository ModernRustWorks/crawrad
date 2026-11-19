---
id: targets-makepad-state-field-map
kind: doc
target_file: src/targets/makepad/state.rs
line_hint: 46
author: 
created: 
title: "FIELD_MAP"
tags: []
signature: "pub const FIELD_MAP : & [(& str , & str , Field)] = & [(\"Timer\" , \"interval\" , Field { name : \"interval_ms\" , ty : \"u64\" , } ,) , (\"ImageList\" , \"images_data\" , Field { name : \"images\" , ty : \"Vec<ImageItem>\" , } ,) , (\"ListView\" , \"items\" , Field { name : \"items\" , ty : \"Vec<ListViewItem>\" , } ,) , (\"TreeView\" , \"items\" , Field { name : \"items\" , ty : \"Vec<crawrad_runtime::tree::TreeNode>\" , } ,) , (\"ComboBox\" , \"item_index\" , Field { name : \"selected\" , ty : \"Option<usize>\" , } ,) , (\"ListBox\" , \"item_index\" , Field { name : \"selected\" , ty : \"Option<usize>\" , } ,) , (\"TreeView\" , \"item_index\" , Field { name : \"selected\" , ty : \"Option<usize>\" , } ,) , (\"PageControl\" , \"active_page\" , Field { name : \"active_page\" , ty : \"usize\" , } ,) , (\"ScrollBar\" , \"min\" , Field { name : \"min\" , ty : \"f32\" , } ,) , (\"ScrollBar\" , \"max\" , Field { name : \"max\" , ty : \"f32\" , } ,) , (\"ScrollBar\" , \"position\" , Field { name : \"position\" , ty : \"f32\" , } ,) , (\"ScrollBar\" , \"small_change\" , Field { name : \"small_change\" , ty : \"f32\" , } ,) , (\"ScrollBar\" , \"large_change\" , Field { name : \"large_change\" , ty : \"f32\" , } ,) ,]"
---

Properties that this target stores under another name or type.

**THE VALVE** — properties that this target stores under another name or another
type.

Each line here is a declared exception, and the size of the table is the yardstick: if
it grows with every new class, the convention is wrong. Today they are the ones that the
measurement on 18/09/2026 found, and none was invented.

# Examples

```rust
use crawrad_codegen::targets::makepad::state::FIELD_MAP;

assert!(!FIELD_MAP.is_empty());
```
