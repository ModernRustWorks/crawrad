---
id: welcome-welcomeaction
kind: doc
target_file: src/welcome.rs
line_hint: 237
author: 
created: 
title: "WelcomeAction"
tags: []
signature: "pub enum WelcomeAction"
---

What a click on the Welcome page asks for.

# Examples

```rust
use crawrad_runtime_makepad::welcome::WelcomeAction;

let action = WelcomeAction::NewProject;
assert_eq!(action, WelcomeAction::NewProject);
```
