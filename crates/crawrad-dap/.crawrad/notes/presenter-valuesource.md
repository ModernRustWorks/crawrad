---
id: presenter-valuesource
kind: doc
target_file: src/presenter.rs
line_hint: 17
author: 
created: 
title: "ValueSource"
tags: []
signature: "pub trait ValueSource"
---

What a presenter may ask the debugger while formatting a value.

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
assert!(source.children(0).unwrap().is_empty());
```
