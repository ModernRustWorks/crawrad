---
id: code-action-parse
kind: doc
target_file: src/code_action.rs
line_hint: 187
author: 
created: 
title: "parse"
tags: []
signature: "pub fn parse (answer : & Value) -> Vec < CodeAction >"
---

Parses applicable code actions containing valid workspace edits.

The actions of a `(Command | CodeAction)[]` answer that the IDE can
apply: with an `edit` it can read, not `disabled`. A `Command`, an action
that only runs a command, and an edit with a file operation are left out.

# Examples

```rust
use crawrad_lsp::code_action::parse;
use serde_json::json;

let answer = json!([]);
let actions = parse(&answer);
assert!(actions.is_empty());
```
