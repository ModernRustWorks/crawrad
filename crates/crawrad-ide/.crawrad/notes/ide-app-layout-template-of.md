---
id: ide-app-layout-template-of
kind: doc
target_file: src/ide_app/layout.rs
line_hint: 73
author: 
created: 
title: "template_of"
tags: []
signature: "pub fn template_of (panel : Panel) -> LiveId"
---

The DSL template that draws this panel.

**One template per panel, and the `match` is exhaustive on purpose.**
`Explorer` and `Packages` used to share a `PanelPending` that wrote a
"not wired yet" sentence at runtime; the two got a screen in this round
(issues #121 and #128), and the shared template went with them. What the
exhaustive `match` buys is the thing the shared one gave away: a panel
added to the catalogue does not compile until somebody chooses what draws
it — instead of silently inheriting a notice panel nobody notices is still
there.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let tmpl = template_of(crawrad_actions::Panel::Inspector);
assert_ne!(tmpl, LiveId(0));
```
