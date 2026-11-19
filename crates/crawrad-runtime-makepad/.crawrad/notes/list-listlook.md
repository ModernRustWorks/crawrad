---
id: list-listlook
kind: doc
target_file: src/list.rs
line_hint: 133
author: 
created: 
title: "ListLook"
tags: []
signature: "pub struct ListLook"
---

Font size and text and background colours of a [`CrawradListBox`].

The font size and the text and background colours of a [`CrawradListBox`]
(#433). A size of 0 and an alpha of 0 mean the theme's.

# Examples

```rust
use crawrad_runtime_makepad::list::ListLook;

let look = ListLook::default();
assert_eq!(look.font_size, 0.0);
```
