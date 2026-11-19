---
id: presenter-valuepresenter
kind: doc
target_file: src/presenter.rs
line_hint: 29
author: 
created: 
title: "ValuePresenter"
tags: []
signature: "pub trait ValuePresenter : Send + Sync"
---

Formats one variable for the screen.

# Examples

```rust
use crawrad_dap::{AdapterText, DapError, ValuePresenter, ValueSource, Variable};

struct DummySource;
impl ValueSource for DummySource {
    fn children(&self, _ref: i64) -> Result<Vec<Variable>, DapError> { Ok(Vec::new()) }
    fn read_memory(&self, _addr: &str, _count: usize) -> Result<Vec<u8>, DapError> { Ok(Vec::new()) }
    fn evaluate(&self, _expr: &str, _frame: Option<i64>) -> Result<Variable, DapError> {
        Err(DapError::Closed)
    }
}

let presenter = AdapterText;
let var = Variable {
    name: "x".into(),
    value: "10".into(),
    type_name: None,
    variables_reference: 0,
    evaluate_name: None,
    memory_reference: None,
};
assert_eq!(presenter.present(&var, &DummySource), None);
```
