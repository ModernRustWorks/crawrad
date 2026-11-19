---
id: presenter-valuepresenter-present
kind: doc
target_file: src/presenter.rs
line_hint: 31
author: 
created: 
title: "ValuePresenter::present"
tags: []
signature: "fn present (& self , variable : & Variable , source : & dyn ValueSource) -> Option < String >"
---

Formats a variable as readable text, or None to keep the adapter output.

The readable text for `variable`, or `None` to show what the adapter
sent. `None` is the honest answer for a type the presenter does not
know yet — never a guess.

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
assert!(presenter.present(&var, &DummySource).is_none());
```
