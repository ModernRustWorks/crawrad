---
id: ide-app-keyboard-comandos
kind: doc
target_file: src/ide_app/keyboard.rs
line_hint: 14
author: 
created: 
title: "comandos"
tags: []
signature: "pub fn comandos (tecla : KeyCode , m : & KeyModifiers , do_campo : bool) -> Vec < Comando >"
---

Returns the commands fired by a keyboard shortcut.

The commands a key fires, in the active scheme or else the other one:
see [`key_routing::commands`].

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mods = makepad_widgets::KeyModifiers::default();
let cmds = comandos(makepad_widgets::KeyCode::Escape, &mods, false);
assert!(!cmds.is_empty());
```
