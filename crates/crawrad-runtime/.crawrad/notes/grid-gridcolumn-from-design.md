---
id: grid-gridcolumn-from-design
kind: doc
target_file: src/grid.rs
line_hint: 114
author: 
created: 
title: "GridColumn::from_design"
tags: []
signature: "pub fn from_design (names : & [String] , numeric : & [bool] , specs : & [ColumnSpec]) -> Vec < GridColumn >"
---

The columns a grid shows, from its saved column specs (#936).

With no `specs`, every field of the table with a title made from its name
(numbers to the right); otherwise one column per visible spec whose field
exists, in the specs' order.

`source` is the field's place in the row, so a hidden or reordered column
still reads the right cell.

# Examples

```rust
use crawrad_runtime::grid::{ColumnSpec, GridColumn};

let names = vec!["id".to_string(), "name".to_string()];
let spec = ColumnSpec {
    field: "name".into(),
    title: String::new(),
    width: 0,
    align: String::new(),
    display_format: String::new(),
    visible: true,
    title_alignment: String::new(),
    min_width: 0,
    max_width: 0,
    null_text: String::new(),
    boolean_texts: String::new(),
    value_map: String::new(),
    renderer: "crText".into(),
    color: crawrad_ir::Color::transparent(),
    text_color: crawrad_ir::Color::transparent(),
    title_color: crawrad_ir::Color::transparent(),
    vertical_alignment: "tlCenter".into(),
    padding: crawrad_ir::Edges::uniform(0.0),
    ellipsis: false,
    word_wrap: false,
    title_word_wrap: false,
    title_hint: String::new(),
    cell_hint: false,
    title_font: crawrad_ir::Font::default(),
    font: crawrad_ir::Font::default(),
    auto_size: false,
    resizable: true,
    movable: true,
    sortable: true,
    sort_order: "soNone".into(),
    sort_priority: 0,
    nulls_position: "npLast".into(),
    groupable: true,
    group_index: 0,
    summary: "smNone".into(),
    summary_format: String::new(),
    group_summary: "smNone".into(),
    filterable: false,
    filter_kind: "fkAuto".into(),
    read_only: false,
    required: false,
    default_value: String::new(),
    max_length: 0,
    min_value: 0,
    max_value: 0,
    tab_stop: true,
    editor: "edAuto".into(),
    pick_list: Vec::new(),
    edit_mask: String::new(),
    get_text_hook: false,
    set_text_hook: false,
    validate_hook: false,
    lookup_key: String::new(),
    lookup_display: Vec::new(),
    button_hook: false,
    drop_down_count: 8,
    field_type: "UNKNOWN".into(),
    draw_cell_hook: false,
    style_rules: Vec::new(),
};
let cols = GridColumn::from_design(&names, &[true, false], &[spec]);
assert_eq!(cols.len(), 1);
assert_eq!(cols[0].title, "Name");
assert_eq!(cols[0].source, Some(1));
```
