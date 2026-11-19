---
id: components-date-time-picker-date-time-picker
kind: doc
target_file: src/components/date_time_picker.rs
line_hint: 4
author: 
created: 
title: "date_time_picker"
tags: []
signature: "pub fn date_time_picker () -> ComponentDesc"
---

`DateTimePicker` (#21): the date and time entry box.

In date mode (`dtkDate`) the box displays `dd/mm/yyyy` and holds canonical
ISO `YYYY-MM-DD`. In time mode (`dtkTime`) it displays `HH:MM:SS` and holds
canonical ISO `HH:MM:SS`.

Default today is NOT allowed in the designer (it changes the form file every
day): default is fixed to "2000-01-01" / "00:00:00".

The value `date_time` is not offered for data binding in this issue.

# Examples

```rust
use crawrad_registry::date_time_picker;

let desc = date_time_picker();
assert_eq!(desc.class, "DateTimePicker");
```
