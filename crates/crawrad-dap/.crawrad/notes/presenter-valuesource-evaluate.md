---
id: presenter-valuesource-evaluate
kind: doc
target_file: src/presenter.rs
line_hint: 25
author: 
created: 
title: "ValueSource::evaluate"
tags: []
signature: "fn evaluate (& self , expression : & str , frame_id : Option < i64 >) -> Result < Variable , DapError >"
---

An expression evaluated in the frame `frame_id` (`evaluate`, `watch`).

# Examples

```rust
use crawrad_dap::{DapError, ValueSource, Variable};

struct DummySource;
impl ValueSource for DummySource {
    fn children(&self, _ref: i64) -> Result<Vec<Variable>, DapError> { Ok(Vec::new()) }
    fn read_memory(&self, _addr: &str, _count: usize) -> Result<Vec<u8>, DapError> { Ok(Vec::new()) }
    fn evaluate(&self, expr: &str, _frame: Option<i64>) -> Result<Variable, DapError> {
        Ok(Variable {
            name: expr.into(),
            value: "42".into(),
            type_name: Some("i32".into()),
            variables_reference: 0,
            evaluate_name: None,
            memory_reference: None,
        })
    }
}

let source = DummySource;
assert_eq!(source.evaluate("x", None).unwrap().value, "42");
```

# Errors

Returns [`DapError`] if expression evaluation fails or times out.
