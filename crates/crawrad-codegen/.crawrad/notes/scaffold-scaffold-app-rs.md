---
id: scaffold-scaffold-app-rs
kind: doc
target_file: src/scaffold.rs
line_hint: 34
author: 
created: 
title: "scaffold_app_rs"
tags: []
signature: "pub fn scaffold_app_rs (alvo : AlvoGerador , nome : & str , template_id : & str) -> Option < String >"
---

Returns the contents of `src/app.rs` suitable for the template and target.

Returns the contents of `src/app.rs` suitable for the template (for targets
based on an app module).

# Examples

```rust
use crawrad_codegen::{AlvoGerador, scaffold::scaffold_app_rs};

let app = scaffold_app_rs(AlvoGerador::Makepad, "meu_app", "wasm_app");
assert!(app.is_some());
```
