---
id: capability-capability-execute
kind: doc
target_file: src/capability.rs
line_hint: 409
author: 
created: 
title: "Capability::execute"
tags: []
signature: "pub fn execute (& self , ctx : & Context < '_ >) -> Result < Effect , CapabilityError >"
---

Executes the capability directly, without registry validations.

# Errors

Returns [`CapabilityError`] if the underlying executor fails.

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
assert!(cap.execute(&ctx).is_ok());
```
