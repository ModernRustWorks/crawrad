---
id: symbols-parse
kind: doc
target_file: src/symbols.rs
line_hint: 72
author: 
created: 
title: "parse"
tags: []
signature: "pub fn parse (value : serde_json :: Value) -> Vec < Symbol >"
---

Parses workspace symbols from an LSP JSON array response.

The items of a `SymbolInformation[]` or `WorkspaceSymbol[]` answer; one
without a name or a file is skipped.

# Examples

```rust
use crawrad_lsp::symbols::parse;
use serde_json::json;

let val = json!([{
    "name": "MyStruct",
    "kind": 23,
    "location": {
        "uri": "file:///path/to/file.rs",
        "range": { "start": { "line": 5, "character": 0 } }
    }
}]);
let syms = parse(val);
assert_eq!(syms.len(), 1);
assert_eq!(syms[0].name, "MyStruct");
```
