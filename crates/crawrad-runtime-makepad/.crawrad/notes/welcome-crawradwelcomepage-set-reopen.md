---
id: welcome-crawradwelcomepage-set-reopen
kind: doc
target_file: src/welcome.rs
line_hint: 339
author: 
created: 
title: "CrawradWelcomePage::set_reopen"
tags: []
signature: "pub fn set_reopen (& mut self , cx : & mut Cx , reopen : bool)"
---

The `Reopen the last project` box and the note that explains it.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use crawrad_runtime_makepad::welcome::CrawradWelcomePage;

# fn demo(page: &mut CrawradWelcomePage, cx: &mut makepad_widgets::Cx) {
page.set_reopen(cx, true);
# }
```
