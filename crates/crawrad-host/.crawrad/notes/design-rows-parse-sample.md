---
id: design-rows-parse-sample
kind: doc
target_file: src/design_rows.rs
line_hint: 223
author: 
created: 
title: "parse_sample"
tags: []
signature: "pub fn parse_sample(text: &str) -> Answer"
---

Reads the probe's `{columns, rows, total}` answer, or its `{error}`.

# Examples

```rust
use crawrad_host::design_rows::parse_sample;

let s = parse_sample(r#"{"columns":["id"],"rows":[["1"]],"total":1}"#).unwrap();
assert_eq!(s.total, 1);
```

# Errors

When the text is not JSON or carries the probe's `error` instead of rows.
