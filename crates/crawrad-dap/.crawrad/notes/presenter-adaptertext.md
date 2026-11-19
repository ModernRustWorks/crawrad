---
id: presenter-adaptertext
kind: doc
target_file: src/presenter.rs
line_hint: 35
author: 
created: 
title: "AdapterText"
tags: []
signature: "pub struct AdapterText"
---

The presenter that adds nothing: the adapter's own text.

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

let var = Variable {
    name: "a".into(),
    value: "1".into(),
    type_name: None,
    variables_reference: 0,
    evaluate_name: None,
    memory_reference: None,
};
assert!(AdapterText.present(&var, &DummySource).is_none());
```
