---
id: capability-capabilityregistry-execute
kind: doc
target_file: src/capability.rs
line_hint: 797
author: 
created: 
title: "CapabilityRegistry::execute"
tags: []
signature: "pub fn execute (& self , id : & str , ctx : & Context < '_ > , active_view : Option < ViewAffinity > ,) -> Result < Effect , CapabilityError >"
---

Executes a capability directly, validating parameters and view affinity.

Executes a capability directly and atomically, validating parameters and view affinity.

# Errors

Returns [`CapabilityError`] if the capability is not found, disabled,
missing required parameters, has an incompatible view affinity, or if
the executor fails.

# Examples

```rust
use crawrad_actions::capability::{Capability, CapabilityRegistry, CapabilitySource};
use crawrad_actions::{Args, Context, Effect, Project};
use crawrad_designer::clipboard::Clipboard;
use crawrad_doc::Author;
use crawrad_registry::Registry;

let mut reg = CapabilityRegistry::new();
let cap = Capability::custom("test.cmd", "Test", "Desc", "Group", CapabilitySource::Native, |_| Ok(Effect::none()));
reg.register(cap).unwrap();
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
assert!(reg.execute("test.cmd", &ctx, None).is_ok());
```
