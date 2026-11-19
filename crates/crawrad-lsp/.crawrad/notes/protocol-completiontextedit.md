---
id: protocol-completiontextedit
kind: doc
target_file: src/protocol.rs
line_hint: 308
author: 
created: 
title: "CompletionTextEdit"
tags: []
signature: "pub struct CompletionTextEdit"
---

The text payload of a completion edit without fixed range constraints.

The text of a `TextEdit` or an `InsertReplaceEdit`; the range is the
editor's to decide.

# Examples

```rust
use crawrad_lsp::protocol::CompletionTextEdit;

let edit = CompletionTextEdit { new_text: "foo".to_string() };
assert_eq!(edit.new_text, "foo");
```
