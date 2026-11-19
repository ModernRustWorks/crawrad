---
id: props-props-tab-visible
kind: doc
target_file: src/props.rs
line_hint: 861
author: 
created: 
title: "Props::tab_visible"
tags: []
signature: "pub fn tab_visible (default : bool) -> PropDesc"
---

Whether a page's tab is shown (`TabVisible`, #449).

Whether a page's tab is shown (`TabVisible`, #449): a hidden tab's page
is reached only by `ActivePageIndex`.

# Examples

```rust
use crawrad_registry::Props;

let p = Props::tab_visible(true);
assert_eq!(p.key, "tab_visible");
```
