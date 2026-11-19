---
id: targets-shared-peca-filepath
kind: doc
target_file: src/targets/shared.rs
line_hint: 200
author: 
created: 
title: "PECA_FILEPATH"
tags: []
signature: "pub const PECA_FILEPATH : & str = include_str ! (\"../../../crawrad-component/src/pieces/file_path.rs\")"
---

O texto da peca `FilePath`.

# Examples

```rust
use crawrad_codegen::targets::shared::PECA_FILEPATH;

assert!(PECA_FILEPATH.contains("struct FilePath"));
```
