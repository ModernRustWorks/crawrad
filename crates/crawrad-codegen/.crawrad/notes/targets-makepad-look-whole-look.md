---
id: targets-makepad-look-whole-look
kind: doc
target_file: src/targets/makepad/look.rs
line_hint: 119
author: 
created: 
title: "whole_look"
tags: []
signature: "pub fn whole_look (f : & str , widget : & str) -> String"
---

Emits synchronization lines applying font and color look to a custom widget.

The sync lines that hand a widget of our own (`widget` is its accessor)
the whole look in one call: it styles each row itself. A component that
draws with one of these widgets calls this from its own sync (#448: the
TreeView with the ListBox's).

# Examples

```rust
use crawrad_codegen::targets::makepad::look::whole_look;

let code = whole_look("list1", "crawrad_list_box");
assert!(code.contains("set_look"));
```
