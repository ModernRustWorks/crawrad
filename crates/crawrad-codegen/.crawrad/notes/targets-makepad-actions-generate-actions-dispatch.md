---
id: targets-makepad-actions-generate-actions-dispatch
kind: doc
target_file: src/targets/makepad/actions.rs
line_hint: 8
author: 
created: 
title: "generate_actions_dispatch"
tags: []
signature: "pub fn generate_actions_dispatch (reg : & crawrad_registry :: Registry , doc : & Document , emitted : & [& Node] ,) -> Result < String , Erro >"
---

The form's `handle_actions`: one query to the framework per bound event.

**It receives the CATALOG**, and this is not a signature detail: without it, the
component that came from a `manifest.json` cannot be recognized, and the
event that the user bound to it comes out of here as an empty string — the handler
is born in their file and is never called.

# Examples

```rust
use crawrad_codegen::targets::makepad::actions::generate_actions_dispatch;
use crawrad_ir::Document;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let doc = Document::new("MainForm", "Form");
let code = generate_actions_dispatch(&reg, &doc, &[]).unwrap();
assert!(code.is_empty());
```

# Errors

Returns an error if an identifier name is invalid or action emission fails.
