---
id: targets-egui-state-field-docs
kind: doc
target_file: src/targets/egui/state.rs
line_hint: 274
author: 
created: 
title: "FIELD_DOCS"
tags: []
signature: "pub const FIELD_DOCS : & [(& str , & str , & str)] = & [(\"\" , \"cursor\" , \"The mouse pointer over the control (`Cursor`, #667).\" ,) , (\"BitBtn\" , \"glyph\" , \"The picture left of the caption (#454); `num_glyphs` pictures side by side,\\nthe first one shown.\" ,) , (\"SpeedButton\" , \"glyph\" , \"The picture left of the caption (#454); `num_glyphs` pictures side by side,\\nthe first one shown.\" ,) , (\"SpeedButton\" , \"flat\" , \"Without a frame until the pointer is over it (#454).\" ,) , (\"SpeedButton\" , \"down\" , \"Pressed: in a group, the one that is down.\" ,) , (\"SpeedButton\" , \"group_index\" , \"Its group among the SpeedButtons of the same parent; 0 = none.\" ,) , (\"SpeedButton\" , \"allow_all_up\" , \"The one that is down can be lifted, leaving the group all up.\" ,) , (\"ToolBar\" , \"flat\" , \"The bar's SpeedButtons draw flat, and show their caption (#457).\" ,) , (\"StatusBar\" , \"simple_panel\" , \"On: the bar shows `simple_text`; off: its panels.\" ,) , (\"Shape\" , \"brush_color\" , \"The fill (`Brush.Color`) and the outline (`Pen.Color`), #460.\" ,) , (\"Splitter\" , \"offset\" , \"How far it was dragged, in points (#461); its neighbours add it.\" ,) , (\"Edit\" , \"char_case\" , \"The case the typed text is held to (`CharCase`, #667).\" ,) , (\"MaskEdit\" , \"char_case\" , \"The case the typed text is held to (`CharCase`, #667).\" ,) , (\"MaskEdit\" , \"edit_mask\" , \"The mask the text is shaped by (`EditMask`, #71): `0` digit, `L`\nletter, `>`/`<` case… — see `crawrad_runtime::mask`.\" ,) , (\"DateTimePicker\" , \"kind\" , \"The entry and display mode (`Kind`, #21): `dtkDate` or `dtkTime`.\" ,) , (\"DateTimePicker\" , \"date_time\" , \"The current value in ISO format (`DateTime`, #21): `YYYY-MM-DD` or `HH:MM:SS`.\" ,) , (\"DateTimePicker\" , \"min_date\" , \"The minimum allowed value in ISO format (`MinDate`, #21).\" ,) , (\"DateTimePicker\" , \"max_date\" , \"The maximum allowed value in ISO format (`MaxDate`, #21).\" ,) , (\"DateTimePicker\" , \"shown\" , \"The text currently shown in the box (#21).\" ,) , (\"CheckBox\" , \"state\" , \"`cbUnchecked`, `cbChecked` or `cbGrayed` (#445); `checked` is the\\ntruth for on/off, and only `state` can say \\\"grayed\\\".\" ,) , (\"CheckBox\" , \"allow_grayed\" , \"A click can reach `cbGrayed`.\") , (\"CheckBox\" , \"alignment\" , \"`taRightJustify` (the caption right of the box) or `taLeftJustify`.\" ,) , (\"ListBox\" , \"selection\" , \"Every row marked: with `multi_select` off, `selected` or nothing (#439).\" ,) , (\"ListBox\" , \"sorted\" , \"Keeps `items` in alphabetical order, ignoring case (#439).\" ,) , (\"ComboBox\" , \"sorted\" , \"Keeps `items` in alphabetical order, ignoring case (#440).\" ,) , (\"Grid\" , \"table_name\" , \"The table a grid bound to a `Connection` shows (#442).\" ,) , (\"PageControl\" , \"tab_position\" , \"`tpTop` or `tpBottom`: where the tabs sit (#449).\" ,) , (\"TabSheet\" , \"tab_visible\" , \"The page's tab is shown (#449); a hidden one is reached by `active_page`.\" ,) , (\"TreeView\" , \"collapsed\" , \"The paths of the folded nodes: their children are not shown.\" ,) ,]"
---

Per-field doc comments preserved on generated struct fields.

# Examples

```rust
use crawrad_codegen::targets::egui::state::FIELD_DOCS;

assert!(!FIELD_DOCS.is_empty());
```
