---
id: capability-capabilityregistry-dispatch
kind: doc
target_file: src/capability.rs
line_hint: 857
author: 
created: 
title: "CapabilityRegistry::dispatch"
tags: []
signature: "pub fn dispatch (& mut self , id : & str , ctx : & Context < '_ > , active_view : Option < ViewAffinity > ,) -> Result < Effect , CapabilityError >"
---

Dispatches a capability and records the execution in the audit log.

Dispatches the capability, validating context and recording the entry in the audit log.

# Errors

Returns [`CapabilityError`] if execution fails.

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
assert!(reg.dispatch("test.cmd", &ctx, None).is_ok());
assert_eq!(reg.audit_log().len(), 1);
```
