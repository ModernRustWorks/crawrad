---
id: grid-columns-columnspec
kind: doc
target_file: src/grid_columns.rs
line_hint: 39
author: 
created: 
title: "ColumnSpec"
tags: []
signature: "pub struct ColumnSpec"
---

One `GridColumn` child of a grid as the Designer saved it (#936).

The canvas reads it from the document, the generators write it into the app;
`crawrad_runtime::grid::GridColumn::from_design` turns a list of them into the columns shown.

# Examples

```rust
use crawrad_ir::grid_columns::ColumnSpec;

let spec = ColumnSpec {
    field: "balance".into(),
    title: "Balance".into(),
    width: 90,
    align: "taRightJustify".into(),
    display_format: "#,##0.00".into(),
    visible: true,
    title_alignment: "taLeftJustify".into(),
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
assert_eq!(spec.width, 90);
```
