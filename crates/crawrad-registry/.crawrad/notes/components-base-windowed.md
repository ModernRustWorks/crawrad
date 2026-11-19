---
id: components-base-windowed
kind: doc
target_file: src/components/base.rs
line_hint: 101
author: 
created: 
title: "WINDOWED"
tags: []
signature: "pub const WINDOWED : & [& str] = & [\"BitBtn\" , \"Breadcrumb\" , \"Button\" , \"CheckBox\" , \"CheckListBox\" , \"Chips\" , \"ColorBox\" , \"ComboBox\" , \"DateTimePicker\" , \"Edit\" , \"Grid\" , \"GroupBox\" , \"ListBox\" , \"ListView\" , \"MaskEdit\" , \"Memo\" , \"MultiSelect\" , \"PageControl\" , \"Pagination\" , \"Panel\" , \"ProgressBar\" , \"RadioButton\" , \"RadioGroup\" , \"ScrollBar\" , \"ScrollBox\" , \"SpinEdit\" , \"StatusBar\" , \"ToolBar\" , \"Toggle\" , \"TrackBar\" , \"TreeView\" ,]"
---

The controls that can take the focus.

**The controls that can take the focus** — classic RAD's `TWinControl`.
Each one gets [`windowed_events`]: the mouse, the focus and the keyboard.

# Examples

```rust
use crawrad_registry::WINDOWED;

assert!(WINDOWED.contains(&"Button"));
```
