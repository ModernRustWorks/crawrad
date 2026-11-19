---
id: capability-capability-is-enabled
kind: doc
target_file: src/capability.rs
line_hint: 373
author: 
created: 
title: "Capability::is_enabled"
tags: []
signature: "pub fn is_enabled (& self , ctx : & Context < '_ >) -> bool"
---

Checks whether the capability is enabled for the current context.

# Examples

```rust
use crawrad_actions::capability::{Capability, CapabilitySource};
use crawrad_actions::{Args, Context, Effect, Project};
use crawrad_designer::clipboard::Clipboard;
use crawrad_doc::Author;
use crawrad_registry::Registry;

let cap = Capability::custom("test.cmd", "Test", "Desc", "Group", CapabilitySource::Native, |_| Ok(Effect::none()));
let registry = Registry::builtin();
let clipboard = Clipboard::default();
let ctx = Context {
    doc: None,
    registry: &registry,
    clipboard: &clipboard,
    args: Args::none(),
    project: Project::default(),
    author: Author::User,
    editor_dirty: false,
    text: Default::default(),
};
assert!(cap.is_enabled(&ctx));
```
