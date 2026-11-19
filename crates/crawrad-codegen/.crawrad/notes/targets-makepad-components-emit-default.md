---
id: targets-makepad-components-emit-default
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 226
author: 
created: 
title: "emit_default"
tags: []
signature: "pub fn emit_default (node : & Node) -> String"
---

Emits the default initialization expression for a given node.

# The THIRD-PARTY branch

A class without a handler in this target, but with a declared `crate_module`, is not
unknown: it is a type that lives in a crate. The `Create` of
ADR 0036
becomes `Tipo::default()`, and each `Set` becomes a field of the literal.

**The crate's `Default` comes FIRST, and then the designer writes over it.**
The third-party crate may have a field that the manifest does not declare, and it is the one
that governs its initial value.

This already held when the form was a closed literal with `..Default::default()`.
The form changed on 18/09/2026, when `#[derive(Component)]` began
publishing ALL fields of a component: then the literal was complete,
`..Default::default()` became `needless_update`, and the user's project
**stopped compiling** under `-D warnings` — because of a fix in the manifest.

A form that only works while the manifest is incomplete is a
flawed form. This one works in both cases.

Without a catalog — which is the case for `emit_default` called from inside this
module — what remains is the raw class name, which is what was there before.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::emit_default;
use crawrad_ir::Node;

let node = Node::new("btn", "Button");
let def = emit_default(&node);
assert!(def.contains("Button {"));
```
