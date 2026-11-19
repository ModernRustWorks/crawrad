---
id: targets-makepad-components-page-control-focus-path
kind: doc
target_file: src/targets/makepad/components/page_control.rs
line_hint: 537
author: 
created: 
title: "focus_path"
tags: []
signature: "pub fn focus_path (doc : & Document , node : & Node , field : & str) -> Option < String >"
---

Returns the Tab stop expression for the open tab at runtime.

#139: the Tab stop holds the button of the tab open at run time.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::page_control::focus_path;
use crawrad_ir::{Document, Node};

let doc = Document::new("MainForm", "Form");
let node = Node::new("pc", "PageControl");
let path = focus_path(&doc, &node, "pc").unwrap();
assert!(path.contains("active_page"));
```
