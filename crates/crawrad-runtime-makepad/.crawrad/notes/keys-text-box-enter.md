---
id: keys-text-box-enter
kind: doc
target_file: src/keys.rs
line_hint: 24
author: 
created: 
title: "text_box_enter"
tags: []
signature: "pub fn text_box_enter (cx : & Cx , memos : & [WidgetRef] , edits : & [WidgetRef] , event : & Event ,) -> Option < Event >"
---

Rewrites Return key events for focused Memo or Edit controls.

The key to hand the widgets instead of `event`, when the focused text box
needs the classic Enter; `None` leaves `event` as it is.

# Examples

```rust,no_run
// Makepad Cx and WidgetRef require the Makepad application runtime.
use makepad_widgets::{Cx, Event};
use crawrad_runtime_makepad::keys::text_box_enter;

fn filter_event(cx: &Cx, event: &Event) -> Option<Event> {
    text_box_enter(cx, &[], &[], event)
}
```
