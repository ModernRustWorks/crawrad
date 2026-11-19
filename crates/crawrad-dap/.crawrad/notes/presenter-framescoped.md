---
id: presenter-framescoped
kind: doc
target_file: src/presenter.rs
line_hint: 101
author: 
created: 
title: "FrameScoped"
tags: []
signature: "pub struct FrameScoped < 'a >"
---

A value source scoped to a specific stack frame.

A [`ValueSource`] tied to one stack frame: every `evaluate` without a frame
of its own runs in `frame_id`.

A presenter formats one variable at a time and does not know which frame
the user is looking at; the session does. So the session hands the
presenter this, and a path like `greeting.vec.len` resolves in the right
frame.

# Examples

```rust
use crawrad_dap::{DapError, FrameScoped, ValueSource, Variable};

struct DummySource;
impl ValueSource for DummySource {
    fn children(&self, _ref: i64) -> Result<Vec<Variable>, DapError> { Ok(Vec::new()) }
    fn read_memory(&self, _addr: &str, _count: usize) -> Result<Vec<u8>, DapError> { Ok(Vec::new()) }
    fn evaluate(&self, expr: &str, frame: Option<i64>) -> Result<Variable, DapError> {
        assert_eq!(frame, Some(42));
        Ok(Variable {
            name: expr.into(),
            value: "ok".into(),
            type_name: None,
            variables_reference: 0,
            evaluate_name: None,
            memory_reference: None,
        })
    }
}

let base = DummySource;
let scoped = FrameScoped { source: &base, frame_id: 42 };
assert_eq!(scoped.evaluate("val", None).unwrap().value, "ok");
```
