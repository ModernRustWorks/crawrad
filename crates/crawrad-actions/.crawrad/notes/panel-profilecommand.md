---
id: panel-profilecommand
kind: doc
target_file: src/panel.rs
line_hint: 222
author: 
created: 
title: "ProfileCommand"
tags: []
signature: "pub enum ProfileCommand"
---

Commands operating on named desktop layout profiles.

What to do with a named desktop profile (#555): the window, the docks,
the Inspector split, the editor font and the interface zoom, kept under a
name. `Default` is the IDE as it ships.

# Examples

```rust
use crawrad_actions::ProfileCommand;

let cmd = ProfileCommand::Apply("Default".into());
assert_eq!(cmd, ProfileCommand::Apply("Default".into()));
```
