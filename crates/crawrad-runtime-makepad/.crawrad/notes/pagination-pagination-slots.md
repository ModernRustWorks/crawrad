---
id: pagination-pagination-slots
kind: doc
target_file: src/pagination.rs
line_hint: 69
author: 
created: 
title: "pagination_slots"
tags: []
signature: "pub fn pagination_slots (page_count : usize , page_index : usize , max_buttons : usize , arrows : bool ,) -> Vec < PageSlot >"
---

Pure function computing which page buttons and gap slots are shown (#500).

- `page_count <= max_buttons`: every page.
- Otherwise first and last pages are always shown, current page with neighbours in middle,
  and `Gap` where pages are skipped; exactly `max(max_buttons, 5)` page+gap slots.
- `arrows`: `Prev` first and `Next` last.
- `page_index` past the end is clamped to `page_count - 1`.

# Examples

```rust
use crawrad_runtime_makepad::pagination::{pagination_slots, PageSlot};

let slots = pagination_slots(3, 0, 5, false);
assert_eq!(slots, vec![PageSlot::Page(0), PageSlot::Page(1), PageSlot::Page(2)]);
```
