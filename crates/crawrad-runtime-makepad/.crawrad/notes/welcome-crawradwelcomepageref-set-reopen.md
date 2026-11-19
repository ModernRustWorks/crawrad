---
id: welcome-crawradwelcomepageref-set-reopen
kind: doc
target_file: src/welcome.rs
line_hint: 416
author: 
created: 
title: "CrawradWelcomePageRef::set_reopen"
tags: []
signature: "pub fn set_reopen (& self , cx : & mut Cx , reopen : bool)"
---

The `Reopen the last project` box.

# Examples

```rust,no_run
// Makepad Cx requires the Makepad application runtime.
use crawrad_runtime_makepad::welcome::CrawradWelcomePageRef;

# fn demo(page: &CrawradWelcomePageRef, cx: &mut makepad_widgets::Cx) {
page.set_reopen(cx, true);
# }
```
