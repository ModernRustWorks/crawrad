---
id: navigation-aids-choices-json
kind: doc
target_file: src/navigation.rs
line_hint: 40
author: 
created: 
title: "Aids::choices_json"
tags: []
signature: "pub fn choices_json (& self) -> serde_json :: Value"
---

Returns current choices in JSON format for the active picker.

What MCP reads (`editor.get_choices`): Link item…'s list when it is
up, else Go to Definition's.

# Examples

```rust
use crawrad_host::navigation::Aids;

let aids = Aids::default();
let json = aids.choices_json();
assert_eq!(json["open"], false);
```
