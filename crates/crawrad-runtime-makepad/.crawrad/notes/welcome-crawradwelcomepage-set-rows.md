---
id: welcome-crawradwelcomepage-set-rows
kind: doc
target_file: src/welcome.rs
line_hint: 317
author: 
created: 
title: "CrawradWelcomePage::set_rows"
tags: []
signature: "pub fn set_rows (& mut self , cx : & mut Cx , rows : & [WelcomeRowView])"
---

The recent projects; with none, the page shows its empty box instead.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use crawrad_runtime_makepad::welcome::{CrawradWelcomePage, WelcomeRowView};

# fn demo(page: &mut CrawradWelcomePage, cx: &mut makepad_widgets::Cx) {
page.set_rows(cx, &[WelcomeRowView::default()]);
# }
```
