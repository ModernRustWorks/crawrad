---
id: events-payload-types
kind: doc
target_file: src/events.rs
line_hint: 364
author: 
created: 
title: "payload_types"
tags: []
signature: "pub fn payload_types (reg : & crawrad_registry :: Registry , handlers : & [Handler]) -> String"
---

Returns payload type definitions needed by the given handlers.

The payload type definitions `handlers` need, ready to be written into the
designer of either target — the same text on both sides, byte for byte.

Empty when no handler names one, which is the common case: a form with a
couple of `OnClick`s gets no payload types at all, and therefore no dead
item to silence.

# Examples

```rust
use crawrad_codegen::events::payload_types;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let types = payload_types(&reg, &[]);
assert!(types.is_empty());
```
