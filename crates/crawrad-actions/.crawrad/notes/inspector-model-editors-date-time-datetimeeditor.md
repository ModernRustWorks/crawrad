---
id: inspector-model-editors-date-time-datetimeeditor
kind: doc
target_file: src/inspector_model/editors/date_time.rs
line_hint: 14
author: 
created: 
title: "DateTimeEditor"
tags: []
signature: "pub struct DateTimeEditor"
---

A date and time editor field.

# Examples

```rust
use crawrad_actions::inspector_model::editors::date_time::DateTimeEditor;
use crawrad_actions::inspector_model::editor::PropEditor;
use crawrad_ir::DateTime;

let ed = DateTimeEditor::new(DateTime::epoch());
assert_eq!(ed.template(), "RowText");
```
