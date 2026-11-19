---
id: toolbar-command-widget
kind: doc
target_file: src/toolbar.rs
line_hint: 1
author:
created:
title: "Command::widget"
tags: []
signature: "pub fn widget (self) -> & 'static str"
---

The name of the command's widget in the top bar's DSL.

The shell turns it into the widget path, so the table of which button is
which lives here with the strip, not in a match inside the shell.

# Examples

```rust
use crawrad_project::toolbar::Command;

assert_eq!(Command::Run.widget(), "btn_play");
assert_eq!(Command::StepOut.widget(), "tb_step_out");
```
