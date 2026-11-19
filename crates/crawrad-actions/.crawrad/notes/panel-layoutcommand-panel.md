---
id: panel-layoutcommand-panel
kind: doc
target_file: src/panel.rs
line_hint: 212
author: 
created: 
title: "LayoutCommand::panel"
tags: []
signature: "pub fn panel (self) -> Option < Panel >"
---

Returns the panel targeted by this command, if any.

The panel this command is about, or `None` for the ones that are about
the whole arrangement.

# Examples

```rust
use crawrad_actions::{LayoutCommand, Panel};

assert_eq!(LayoutCommand::Reset.panel(), None);
```
