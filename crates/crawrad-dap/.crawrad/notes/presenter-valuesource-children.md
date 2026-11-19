---
id: presenter-valuesource-children
kind: doc
target_file: src/presenter.rs
line_hint: 19
author: 
created: 
title: "ValueSource::children"
tags: []
signature: "fn children (& self , variables_reference : i64) -> Result < Vec < Variable > , DapError >"
---

The children of a variable (`variables` request).

# Examples

```rust
use crawrad_dap::{DapError, ValueSource, Variable};

struct DummySource;
impl ValueSource for DummySource {
    fn children(&self, _ref: i64) -> Result<Vec<Variable>, DapError> { Ok(Vec::new()) }
    fn read_memory(&self, _addr: &str, _count: usize) -> Result<Vec<u8>, DapError> { Ok(Vec::new()) }
    fn evaluate(&self, _expr: &str, _frame: Option<i64>) -> Result<Variable, DapError> {
        Err(DapError::Closed)
    }
}

let source = DummySource;
assert_eq!(source.children(1).unwrap(), Vec::new());
```

# Errors

Returns [`DapError`] if the debugger fails to query child variables.
