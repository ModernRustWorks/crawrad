---
id: key-routing-commands
kind: doc
target_file: src/key_routing.rs
line_hint: 104
author: 
created: 
title: "commands"
tags: []
signature: "pub fn commands (p : & Press , from_field : bool) -> Vec < Comando >"
---

Translates a key into commands triggered using the active scheme.

The translation of a key into the list of commands that it triggers using the active scheme.
If no command is found in the active scheme, it queries the other scheme as a fallback
(allowing both Ctrl+Shift+B and Shift+F9 to open Build regardless of the active mode).

# Examples

```rust
use crawrad_actions::key_routing::{commands, Press};

let press = Press {
    matches: &|chord| chord == "Ctrl+S",
    arrow: None,
    escape: false,
    shift: false,
    control: true,
    alt: false,
    logo: false,
};
let cmds = commands(&press, false);
assert!(!cmds.is_empty());
```
