---
id: ide-app-layout-tab-of
kind: doc
target_file: src/ide_app/layout.rs
line_hint: 58
author: 
created: 
title: "tab_of"
tags: []
signature: "pub fn tab_of (panel : Panel) -> LiveId"
---

The `LiveId` of the tab that carries this panel.

Stable across restarts on purpose: it is what the saved arrangement writes
down, and `LiveId` hashes the *name*, so renaming `tab_inspector` here
invalidates every arrangement on every machine. That is exactly the
compatibility surface the module doc warns about — and [`decode`] is what
makes it a discarded file instead of a broken window.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let tab = tab_of(crawrad_actions::Panel::Inspector);
assert_ne!(tab, LiveId(0));
```
