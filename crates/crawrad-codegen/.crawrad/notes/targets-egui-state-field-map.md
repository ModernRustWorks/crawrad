---
id: targets-egui-state-field-map
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 19
author: 
created: 
title: "FIELD_MAP"
tags: []
signature: "pub const FIELD_MAP : & [(& str , & str , Field)] = & [(\"ComboBox\" , \"item_index\" , Field { name : \"selected\" , ty : \"Option<usize>\" , } ,) , (\"Grid\" , \"col_count\" , Field { name : \"col_count\" , ty : \"usize\" , } ,) , (\"Grid\" , \"default_col_width\" , Field { name : \"default_col_width\" , ty : \"f32\" , } ,) , (\"Grid\" , \"default_row_height\" , Field { name : \"default_row_height\" , ty : \"f32\" , } ,) , (\"Grid\" , \"fixed_cols\" , Field { name : \"fixed_cols\" , ty : \"usize\" , } ,) , (\"Grid\" , \"fixed_rows\" , Field { name : \"fixed_rows\" , ty : \"usize\" , } ,) , (\"Grid\" , \"row_count\" , Field { name : \"row_count\" , ty : \"usize\" , } ,) , (\"ImageList\" , \"images_data\" , Field { name : \"images\" , ty : \"Vec<ImageItem>\" , } ,) , (\"ListBox\" , \"item_index\" , Field { name : \"selected\" , ty : \"Option<usize>\" , } ,) , (\"ListView\" , \"items\" , Field { name : \"items\" , ty : \"Vec<ListViewItem>\" , } ,) , (\"PageControl\" , \"active_page\" , Field { name : \"active_page\" , ty : \"usize\" , } ,) , (\"Panel\" , \"border_width\" , Field { name : \"border_width\" , ty : \"f32\" , } ,) , (\"ScrollBar\" , \"large_change\" , Field { name : \"large_change\" , ty : \"i64\" , } ,) , (\"ScrollBar\" , \"max\" , Field { name : \"max\" , ty : \"i64\" , } ,) , (\"ScrollBar\" , \"min\" , Field { name : \"min\" , ty : \"i64\" , } ,) , (\"ScrollBar\" , \"position\" , Field { name : \"position\" , ty : \"i64\" , } ,) , (\"ScrollBar\" , \"small_change\" , Field { name : \"small_change\" , ty : \"i64\" , } ,) , (\"Shape\" , \"shape\" , Field { name : \"shape_kind\" , ty : \"String\" , } ,) , (\"Timer\" , \"interval\" , Field { name : \"interval\" , ty : \"u64\" , } ,) , (\"TreeView\" , \"items\" , Field { name : \"items\" , ty : \"Vec<crawrad_runtime::tree::TreeNode>\" , } ,) ,]"
---

Renamed or retyped properties in the egui state.

# Examples

```rust
use crawrad_codegen::targets::egui::state::FIELD_MAP;

assert!(!FIELD_MAP.is_empty());
```
