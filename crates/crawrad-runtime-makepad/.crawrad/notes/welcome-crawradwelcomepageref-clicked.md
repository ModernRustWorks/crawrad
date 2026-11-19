---
id: welcome-crawradwelcomepageref-clicked
kind: doc
target_file: src/welcome.rs
line_hint: 429
author: 
created: 
title: "CrawradWelcomePageRef::clicked"
tags: []
signature: "pub fn clicked (& self , actions : & Actions) -> Option < WelcomeAction >"
---

What a click asked for.

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use crawrad_runtime_makepad::welcome::CrawradWelcomePageRef;

# fn demo(page: &CrawradWelcomePageRef, actions: &makepad_widgets::Actions) {
let action = page.clicked(actions);
assert!(action.is_none());
# }
```
