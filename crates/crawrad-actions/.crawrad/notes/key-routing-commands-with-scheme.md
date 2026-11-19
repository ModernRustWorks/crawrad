---
id: key-routing-commands-with-scheme
kind: doc
target_file: src/key_routing.rs
line_hint: 118
author: 
created: 
title: "commands_with_scheme"
tags: []
signature: "pub fn commands_with_scheme (p : & Press , from_field : bool , scheme : KeymapScheme) -> Vec < Comando >"
---

Translates a key into commands triggered in a specific scheme.

The translation of a key into the list of commands that it triggers for a given scheme.

# Examples

```rust
use crawrad_actions::key_routing::{commands_with_scheme, Press};
use crawrad_actions::KeymapScheme;

let press = Press {
    matches: &|chord| chord == "Ctrl+S",
    arrow: None,
    escape: false,
    shift: false,
    control: true,
    alt: false,
    logo: false,
};
let cmds = commands_with_scheme(&press, false, KeymapScheme::VsCode);
assert!(!cmds.is_empty());
```
