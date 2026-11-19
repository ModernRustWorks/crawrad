---
id: events-payload-types-excluding
kind: doc
target_file: src/events.rs
line_hint: 369
author: 
created: 
title: "payload_types_excluding"
tags: []
signature: "pub fn payload_types_excluding (reg : & crawrad_registry :: Registry , handlers : & [Handler] , excluding : & [& 'static str] ,) -> String"
---

Returns payload type definitions needed by handlers, excluding given types.

The payload type definitions `handlers` need, excluding any types already emitted
by pieces (such as `Key` emitted by `Shortcut`).

# Examples

```rust
use crawrad_codegen::events::payload_types_excluding;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let types = payload_types_excluding(&reg, &[], &["Key"]);
assert!(types.is_empty());
```
