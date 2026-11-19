---
id: welcome-crawradwelcomepage-clicked
kind: doc
target_file: src/welcome.rs
line_hint: 372
author: 
created: 
title: "CrawradWelcomePage::clicked"
tags: []
signature: "pub fn clicked (& self , actions : & Actions) -> Option < WelcomeAction >"
---

What the user's click asked for, if anything.

# Examples

```rust,no_run
// Makepad Actions require the Makepad application runtime.
use crawrad_runtime_makepad::welcome::CrawradWelcomePage;

# fn demo(page: &CrawradWelcomePage, actions: &makepad_widgets::Actions) {
let action = page.clicked(actions);
assert!(action.is_none());
# }
```
