---
id: targets-shared-look-consts
kind: doc
target_file: src/targets/shared.rs
line_hint: 394
author: 
created: 
title: "look_consts"
tags: []
signature: "pub fn look_consts (doc : & crawrad_ir :: Document , reg : & crawrad_registry :: Registry , emitted : & [& crawrad_ir :: Node]) -> String"
---

Emits font size and color constants from the form for child controls.

The form's font size as a constant (#433), for a control whose
`ParentFont` is on — the same text on both targets.
Each is written only when some EMITTED control reads it (an unread constant
is a warning in the user's project): `font` among its class's catalogue
properties. A derived form's override entry (#851) is not emitted, so it does
not count. `ParentColor` needs none: it paints nothing.

# Examples

```rust
use crawrad_codegen::targets::shared::look_consts;
use crawrad_ir::Document;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let doc = Document::new("form", "Form");
let code = look_consts(&doc, &reg, &[]);
assert!(code.is_empty());
```
