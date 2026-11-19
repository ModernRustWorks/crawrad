---
id: targets-egui-layout-emitido-no
kind: doc
target_file: src/targets/egui/layout.rs
line_hint: 20
author: 
created: 
title: "emitido_no"
tags: []
signature: "pub fn emitido_no (doc : & Document , n : & Node) -> bool"
---

Checks whether the node becomes code in the generated application.

**This node** becomes code in the generated application.

The class alone is not enough to answer in one case, and that is `TabSheet`: the catalog
itself states explicitly that *"a page outside of a page control
is nothing at all"* — it is omitted from the palette because of this, its rectangle
is derived from the `PageControl` that it lacks, and it does not draw itself.

Emitting one like that produced a field and a type that **no line in the application
ever read** — the caption goes to the `PageControl` tab strip, and without a `PageControl` there
is no tab strip. It was genuine `dead_code`, and was silenced for years by an
`#![allow]`. Not emitting is the root fix: the design already showed
nothing, and now the code does not pretend to show anything either.

# Examples

```rust
use crawrad_codegen::targets::egui::layout::emitido_no;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("btn", "Button");
assert!(emitido_no(&doc, &node));
```
