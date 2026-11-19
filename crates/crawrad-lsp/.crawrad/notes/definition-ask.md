---
id: definition-ask
kind: doc
target_file: src/definition.rs
line_hint: 147
author: 
created: 
title: "ask"
tags: []
signature: "pub fn ask (request : & mut dyn FnMut (Duration) -> Result < Vec < Location > , crate :: LspError > , enc : crate :: encoding :: PositionEncoding , read : & mut dyn FnMut (& Path) -> Option < String > , timeout : Duration , retry : Duration ,) -> Outcome"
---

Performs a definition request with retry, returning resolved targets.

One definition request, [`retrying`] on `ContentModified`, its places
in the editor's unit.

# Examples

```rust
use crawrad_lsp::definition::{ask, Outcome};
use crawrad_lsp::encoding::PositionEncoding;
use std::time::Duration;

let mut req = |_left| Ok(Vec::new());
let outcome = ask(&mut req, PositionEncoding::Utf8, &mut |_| None, Duration::from_secs(1), Duration::from_millis(10));
assert_eq!(outcome, Outcome::Nothing);
```
