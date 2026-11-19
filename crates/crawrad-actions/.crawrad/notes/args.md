---
id: args
kind: doc
target_file: src/lib.rs
line_hint: 193
author: 
created: 
title: "Args"
tags: []
signature: "pub struct Args"
---

The named arguments of an action mapped to values.

The arguments of an action — **name → [`Value`]** —, and the origin of the
MCP tool signature.

It used to be positional `Vec<String>`, and the type was the measured block: the literal
example of ADR 0005,
`component.add(class, x, y)`, **did not fit** — `x` and `y` are `f32`. With the
argument bound to text, the four mutations that count (create component,
change property, rename, wire handler) had no way to enter the
catalogue, and kept living in the shell. It is the classic OTA sin that
§12.3 of the study names: the agent aligns, copies and deletes, and cannot
create a button.

[`Value`] and not a new type, for three reasons:

1. it is **closed** — the set of shapes an argument can have is the same
   a property can have, and there is no second vocabulary to
   keep in sync with `crawrad-ir`;
2. it is **serializable** — `Args` is a `BTreeMap<String, Value>`, which is
   exactly the `params` of a JSON-RPC call of §12.3. The second
   consumer is already designed, so this is no guess;
3. the **name** matches the [`Param::name`] that the action publishes. The signature
   that the MCP shows and the key that `execute` reads become the same
   string, instead of a position only the body of the function knew.

Without `Eq`: [`Value`] carries `f64`. It never could have `Eq`, and an
argument that is a coordinate is precisely the case that motivated the change.

# Examples

```rust
use crawrad_actions::Args;

let args = Args::text("class", "Button");
assert_eq!(args.as_text("class"), Some("Button"));
```
