---
id: events-key-variants
kind: doc
target_file: src/events.rs
line_hint: 394
author: 
created: 
title: "key_variants"
tags: []
signature: "pub fn key_variants () -> Vec < & 'static str >"
---

Returns the variant names defined in the `Key` payload type.

**The names in the [`Key`](PAYLOAD_TABLE) vocabulary**, read out of the
very text the designer emits.

A target needs this list to write the conversion from its framework's key
type to ours, and it must be the SAME list: a conversion that names a key
the contract does not have breaks the user's build, and one that forgets a
key reports `Unknown` for a key we do have. Parsing the single definition
makes both impossible — there is nothing to keep in step.

# Examples

```rust
use crawrad_codegen::events::key_variants;

let variants = key_variants();
assert!(variants.contains(&"Enter"));
```
