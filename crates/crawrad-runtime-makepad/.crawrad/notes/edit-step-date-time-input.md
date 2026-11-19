---
id: edit-step-date-time-input
kind: doc
target_file: src/edit.rs
line_hint: 103
author: 
created: 
title: "step_date_time_input"
tags: []
signature: "pub fn step_date_time_input (cx : & mut Cx , input : & TextInputRef , event : & Event , kind : & str , date_time : & mut String , min_date : & str , max_date : & str ,) -> bool"
---

Steps the date or time field under the caret on `ArrowUp` or `ArrowDown` (#21).

Returns `true` if the event was an `ArrowUp` or `ArrowDown` key event while
this input had keyboard focus and stepped the value.

# Examples

```rust,no_run
// Makepad Cx and TextInputRef require the Makepad application runtime.
use makepad_widgets::{Cx, Event, TextInputRef};
use crawrad_runtime_makepad::edit::step_date_time_input;

fn example(cx: &mut Cx, input: &TextInputRef, event: &Event, dt: &mut String) {
    step_date_time_input(cx, input, event, "Date", dt, "", "");
}
```
