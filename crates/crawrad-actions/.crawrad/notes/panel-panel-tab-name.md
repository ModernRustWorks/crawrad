---
id: panel-panel-tab-name
kind: doc
target_file: src/panel.rs
line_hint: 82
author: 
created: 
title: "Panel::tab_name"
tags: []
signature: "pub fn tab_name (self) -> String"
---

Returns the stable tab name for a panel in a saved arrangement.

The stable identifier — what the MCP passes as the `panel` argument,
and what the persisted arrangement writes to disk.

It is **not** the label: a label is allowed to be retitled, and the
day `Components` becomes `Palette` on screen, every saved arrangement
and every agent script has to keep working.
The dock tab's id in a saved arrangement, `tab_<id>`: readable text
there instead of a hash, and a compatibility surface — never rename it.

# Examples

```rust
use crawrad_actions::Panel;

assert_eq!(Panel::Explorer.tab_name(), "tab_explorer");
```
