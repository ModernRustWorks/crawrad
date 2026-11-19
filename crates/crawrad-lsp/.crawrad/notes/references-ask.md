---
id: references-ask
kind: doc
target_file: src/references.rs
line_hint: 95
author: 
created: 
title: "ask"
tags: []
signature: "pub fn ask (request : & mut dyn FnMut (Duration) -> Result < Vec < Location > , crate :: LspError > , timeout : Duration , retry : Duration ,) -> Outcome"
---

One references request, [`retrying`] on `ContentModified`.

# Examples

```rust
use crawrad_lsp::references::{ask, Outcome};
use std::time::Duration;

let mut req = |_| Ok(Vec::new());
let outcome = ask(&mut req, Duration::from_millis(50), Duration::from_millis(10));
assert_eq!(outcome, Outcome::Nothing);
```
