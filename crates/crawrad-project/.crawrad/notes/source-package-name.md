---
id: source-package-name
kind: doc
target_file: src/source.rs
line_hint: 57
author: 
created: 
title: "package_name"
tags: []
signature: "pub fn package_name (form : & Path) -> String"
---

The name of the package the generated code belongs to.

Read from the member's `Cargo.toml`, with `"app"` as the fallback — the
same fallback, and the same reason, as `nome_do_pacote` in the egui shell:
a name the IDE could not read is not a reason to refuse to show the code.

# Examples

```rust
use std::path::Path;
use crawrad_project::source::package_name;

assert_eq!(package_name(Path::new("missing.crawform")), "app");
```
