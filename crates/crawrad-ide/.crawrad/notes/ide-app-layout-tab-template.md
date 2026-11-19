---
id: ide-app-layout-tab-template
kind: doc
target_file: src/ide_app/layout.rs
line_hint: 78
author: 
created: 
title: "TAB_TEMPLATE"
tags: []
signature: "pub const TAB_TEMPLATE : LiveId = live_id ! (PermanentTab)"
---

The tab-label template. One for every panel, and it is not `Closeable`.

Measured, and it is the reason: `Dock::close_tab` with an empty tab group
only unsplits when the group is `closable`
(`makepad-widgets-1.0.0/src/dock.rs:684`), and a closed tab whose group
stays behind leaves an empty column the user cannot get rid of. Closing
goes through `layout.hide_panel`, which closes the tab *and* lets the group
collapse.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
assert_ne!(TAB_TEMPLATE, LiveId(0));
```
