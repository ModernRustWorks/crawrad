---
id: dock-register-tab-names
kind: doc
target_file: src/dock.rs
line_hint: 75
author: 
created: 
title: "register_tab_names"
tags: []
signature: "pub fn register_tab_names < I , S > (names : I) where I : IntoIterator < Item = S > , S : AsRef < str > ,"
---

Teaches the id table the name of every tab.

Teaches the id table the name of every tab, so the saved arrangement is
readable text instead of a wall of numbers.

A collision (the table already holding a DIFFERENT string for that hash)
would make one panel's tab open another; the assertion turns that into a
crash at startup instead of a mystery on screen.

# Panics

Panics if a tab name collides with another already registered name.

# Examples

```rust
use crawrad_runtime_makepad::dock::register_tab_names;

register_tab_names(["tab1", "tab2"]);
```
