---
id: scaffold-scaffold-main-rs
kind: doc
target_file: src/scaffold.rs
line_hint: 6
author: 
created: 
title: "scaffold_main_rs"
tags: []
signature: "pub fn scaffold_main_rs (alvo : AlvoGerador , nome : & str , template_id : & str) -> String"
---

Returns the contents of `src/main.rs` suitable for the template and target.

# Examples

```rust
use crawrad_codegen::{AlvoGerador, scaffold::scaffold_main_rs};

let code = scaffold_main_rs(AlvoGerador::Egui, "meu_app", "console_app");
assert!(code.contains("Hello from meu_app!"));
```
