---
id: presenter-valuesource-read-memory
kind: doc
target_file: src/presenter.rs
line_hint: 22
author: 
created: 
title: "ValueSource::read_memory"
tags: []
signature: "fn read_memory (& self , memory_reference : & str , count : usize) -> Result < Vec < u8 > , DapError >"
---

`count` bytes at `memory_reference`, which may be a plain address.

# Examples

```rust
use crawrad_dap::{DapError, ValueSource, Variable};

struct DummySource;
impl ValueSource for DummySource {
    fn children(&self, _ref: i64) -> Result<Vec<Variable>, DapError> { Ok(Vec::new()) }
    fn read_memory(&self, _addr: &str, count: usize) -> Result<Vec<u8>, DapError> {
        Ok(vec![0; count])
    }
    fn evaluate(&self, _expr: &str, _frame: Option<i64>) -> Result<Variable, DapError> {
        Err(DapError::Closed)
    }
}

let source = DummySource;
assert_eq!(source.read_memory("0x1000", 4).unwrap().len(), 4);
```

# Errors

Returns [`DapError`] if reading memory fails or base64 decoding fails.
