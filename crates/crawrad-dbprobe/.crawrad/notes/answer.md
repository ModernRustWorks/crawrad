---
id: answer
kind: doc
target_file: src/lib.rs
line_hint: 87
author: 
created: 
title: "answer"
tags: []
signature: "pub fn answer (args : & [String]) -> (i32 , String)"
---

Parses CLI arguments and returns an exit code and JSON response.

Pure entrypoint parsing command line arguments (`--url`, `--table`, and optional `--rows`) and producing an exit code and JSON response.

# Examples

```rust
use crawrad_dbprobe::answer;

let (code, json) = answer(&[]);
assert_eq!(code, 1);
assert!(json.contains("missing --url parameter"));

let (code, json) = answer(&["--rows".to_string(), "abc".to_string()]);
assert_eq!(code, 1);
assert!(json.contains("--rows needs a number"));
```
