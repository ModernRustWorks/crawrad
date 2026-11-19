---
id: inspector-model-editors-font-pick-font
kind: doc
target_file: src/inspector_model/editors/font.rs
line_hint: 350
author: 
created: 
title: "pick_font"
tags: []
signature: "pub fn pick_font (doc : Option < & crawrad_ir :: Document > , node : & NodeId , key : & str , font : Option < & str > ,) -> Option < Result < Font , String > >"
---

Resolves the chosen font for the font picker.

What the `…` of a `Font` row writes (#556): the font in `font` when an
agent gives one, else a test's answer ([`crawrad_os::font::test_answer`]: empty
is Cancel), else the native dialog's — started
from the font the document holds. `None` is Cancel; `Err` a font that does
not read.

# Examples

```rust
use crawrad_actions::inspector_model::editors::font::pick_font;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
let res = pick_font(Some(&doc), &"form".to_string(), "font", Some("Arial, 10pt"));
assert!(res.is_some());
```
