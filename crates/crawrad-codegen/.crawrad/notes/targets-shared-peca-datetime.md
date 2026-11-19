---
id: targets-shared-peca-datetime
kind: doc
target_file: src/targets/shared.rs
line_hint: 197
author: 
created: 
title: "PECA_DATETIME"
tags: []
signature: "pub const PECA_DATETIME : & str = include_str ! (\"../../../crawrad-component/src/pieces/date_time.rs\")"
---

O texto da peca `DateTime`.

# Examples

```rust
use crawrad_codegen::targets::shared::PECA_DATETIME;

assert!(PECA_DATETIME.contains("struct DateTime"));
```
