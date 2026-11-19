---
id: code-lens-codelens-to-json
kind: doc
target_file: src/code_lens.rs
line_hint: 40
author: 
created: 
title: "CodeLens::to_json"
tags: []
signature: "pub fn to_json (& self) -> serde_json :: Value"
---

The JSON value sent to `codeLens/resolve`.

# Examples

```rust
use crawrad_lsp::code_lens::CodeLens;
use crawrad_lsp::protocol::{Position, Range};

let lens = CodeLens::new(Range::new(Position::new(0, 0), Position::new(0, 5)), None);
let json = lens.to_json();
assert!(json.get("range").is_some());
```
