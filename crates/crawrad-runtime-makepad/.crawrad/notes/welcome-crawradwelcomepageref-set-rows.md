---
id: welcome-crawradwelcomepageref-set-rows
kind: doc
target_file: src/welcome.rs
line_hint: 409
author: 
created: 
title: "CrawradWelcomePageRef::set_rows"
tags: []
signature: "pub fn set_rows (& self , cx : & mut Cx , rows : & [WelcomeRowView])"
---

The recent projects.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use crawrad_runtime_makepad::welcome::{CrawradWelcomePageRef, WelcomeRowView};

# fn demo(page: &CrawradWelcomePageRef, cx: &mut makepad_widgets::Cx) {
page.set_rows(cx, &[WelcomeRowView::default()]);
# }
```
