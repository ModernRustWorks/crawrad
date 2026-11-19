---
id: targets-makepad-look-sync
kind: doc
target_file: src/targets/makepad/look.rs
line_hint: 87
author: 
created: 
title: "sync"
tags: []
signature: "pub fn sync (doc : & Document , node : & Node , f : & str) -> String"
---

Generates styling sync code for `node` under field name `f`.

The sync lines that style `node` (field `f`), or nothing for a class the
table does not name.

# Examples

```rust
use crawrad_codegen::targets::makepad::look::sync;
use crawrad_ir::{Document, Node};

let doc = Document::new("form", "Form");
let node = Node::new("btn", "Button");
let code = sync(&doc, &node, "btn");
assert!(code.contains("self.form.btn"));
```
