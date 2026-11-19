---
id: dataset-columns-parse-answer
kind: doc
target_file: src/dataset_columns.rs
line_hint: 216
author: 
created: 
title: "parse_answer"
tags: []
signature: "pub fn parse_answer (text : & str) -> Answer"
---

The probe's JSON: `{"columns": [...]}` or `{"error": "..."}`.

# Examples

```rust
use crawrad_host::dataset_columns::parse_answer;

let res = parse_answer(r#"{"columns": ["id", "name"]}"#);
assert_eq!(res, Ok(vec!["id".into(), "name".into()]));
```

# Errors

Returns an error if the input is not JSON or contains an error field.
