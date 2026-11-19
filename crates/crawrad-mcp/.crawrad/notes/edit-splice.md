---
id: edit-splice
kind: doc
target_file: src/edit.rs
line_hint: 18
author: 
created: 
title: "splice"
tags: []
signature: "pub fn splice (current : & str , args : & Value) -> Result < String , Refusal >"
---

Splices replacement text into a line range or replaces whole content.

The new text: lines `start_line..=end_line` (1-based) of `current`
replaced by `new_text`, or the whole `content`. A range past the end
appends; `end_line` past the end is clamped; the file's final newline is
kept as it was.

# Examples

```rust
use crawrad_mcp::edit::splice;
use serde_json::json;

let current = "one\ntwo\nthree\n";
let args = json!({ "start_line": 2, "end_line": 2, "new_text": "replacement" });
let result = splice(current, &args).unwrap();
assert_eq!(result, "one\nreplacement\nthree\n");
```

# Errors

Returns `Err` when line range arguments are invalid or missing required parameters.
