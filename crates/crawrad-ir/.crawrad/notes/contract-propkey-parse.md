---
id: contract-propkey-parse
kind: doc
target_file: src/contract.rs
line_hint: 168
author: 
created: 
title: "PropKey::parse"
tags: []
signature: "pub fn parse (raw : & str) -> Result < Self , ContractError >"
---

Classifies a raw key coming from the file.

# Examples

```rust
use crawrad_ir::PropKey;

let key = PropKey::parse("egui.desired_width").unwrap();
assert_eq!(key, PropKey::Target {
    target: "egui".to_string(),
    name: "desired_width".to_string(),
});
```

# Errors

Returns [`ContractError::MalformedKey`] if `raw` is empty or has an invalid format,
or [`ContractError::UnknownTarget`] if the target prefix is not recognized.

# It NO LONGER decides whether the key exists — [ADR 0031, Amendment 1]

Here once was `if COMMON.contains(&raw)`, and that line was the **only**
real consumer of the list of forty names: it was the existence gatekeeper.

And it was duplication, in the owner's words: *"if we are going to focus on the
component, why is this common needed? That is already being duplicate"*.
The component **already declares** its properties; the list was a
second copy to maintain by hand — and forgetting it already cost dearly (the
`Panel.border_width` was born in the palette with a property that this
`parse` rejected, and every form with a panel failed).

**A key without prefix now comes to mean *every target*, and nothing more.**
What says it EXISTS is the class, and the per-class check lives in
`crawrad-doc` — which sees the catalog, something this crate does not and
must not do.

What remains here is what needs no catalog at all: malformed
keys and unknown targets.

