---
id: ide-app-layout-panel-of-tab
kind: doc
target_file: src/ide_app/layout.rs
line_hint: 68
author: 
created: 
title: "panel_of_tab"
tags: []
signature: "pub fn panel_of_tab (tab : LiveId) -> Option < Panel >"
---

The panel a tab carries, or `None` for an id this build does not know.

`None` is not a bug: it is what an arrangement saved by an older build
looks like after a panel is renamed, and [`decode`] turns it into "use the
default" instead of a tab that draws nothing.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let p = panel_of_tab(tab_of(crawrad_actions::Panel::Inspector));
assert_eq!(p, Some(crawrad_actions::Panel::Inspector));
```
