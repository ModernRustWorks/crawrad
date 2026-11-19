---
id: navigation-choices-to-json
kind: doc
target_file: src/navigation.rs
line_hint: 129
author: 
created: 
title: "Choices::to_json"
tags: []
signature: "pub fn to_json (& self) -> serde_json :: Value"
---

What MCP reads (`editor.get_choices`): open, and the rows' labels.

# Examples

```rust
use crawrad_host::navigation::Choices;

let choices = Choices::default();
let json = choices.to_json();
assert_eq!(json["open"], false);
```
