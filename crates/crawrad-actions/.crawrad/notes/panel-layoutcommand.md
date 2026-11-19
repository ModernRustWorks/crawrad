---
id: panel-layoutcommand
kind: doc
target_file: src/panel.rs
line_hint: 168
author: 
created: 
title: "LayoutCommand"
tags: []
signature: "pub enum LayoutCommand"
---

What the action wants done to the arrangement of the panels.

It is the layout member of the [`Effect`](super::Effect) family, and it is
its own field for the reason that separated `View` from `File`: an
agent reading an effect to know what is about to happen can tell from the
family alone that nothing here writes to disk, launches a process or
touches the document. The `.crawform` comes out byte for byte identical
whatever the panels are doing.

# Examples

```rust
use crawrad_actions::{LayoutCommand, Panel};

let cmd = LayoutCommand::Show(Panel::Explorer);
assert_eq!(cmd.panel(), Some(Panel::Explorer));
```
