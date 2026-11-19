---
id: registry-suggest-id
kind: doc
target_file: src/lib.rs
line_hint: 728
author: 
created: 
title: "Registry::suggest_id"
tags: []
signature: "pub fn suggest_id (& self , class : & str , existing : impl Fn (& str) -> bool) -> String"
---

Suggests an automatic identifier name for a new component.

Automatic name in Rust idiomatic convention: `button1`, `edit2`,
`label3` — [`ComponentDesc::base_name`] followed by the first
free number.

**The name comes from the catalog, not from a conversion.** It used to be
`para_snake_case(d.display)`, and that made automatic case translation
the piece that every component depended on to be born with a valid name.
Today the catalog declares, and a class without `base_name` does not compile.

An unknown class has no base-name to offer, and therefore does not
receive a guessed one: it falls back to `component`, which is what it is, instead
of becoming `Button1`-from-whatever-it-came-from. An unknown class here is already
a caller's defect — what cannot happen is the defect exiting through the other
end as a name that `rustc` blames.

# Examples

```rust
use crawrad_registry::Registry;

let reg = Registry::builtin();
let id = reg.suggest_id("Button", |_| false);
assert_eq!(id, "button1");
```
