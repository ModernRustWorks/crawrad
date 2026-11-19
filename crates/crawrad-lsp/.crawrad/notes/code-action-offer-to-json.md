---
id: code-action-offer-to-json
kind: doc
target_file: src/code_action.rs
line_hint: 97
author: 
created: 
title: "Offer::to_json"
tags: []
signature: "pub fn to_json (& self , place : Option < Place > , bulb : bool) -> Value"
---

Serializes the offer for the editor UI into JSON format.

What `editor.get_code_actions` reads about the caret's `place` (file,
version, 0-based line): its 1-based line, whether it is still asked,
its actions in the order `editor.apply_code_action` takes, the bulb.

# Examples

```rust
use crawrad_lsp::code_action::Offer;

let offer = Offer::default();
let json = offer.to_json(None, false);
assert_eq!(json["waiting"], false);
```
