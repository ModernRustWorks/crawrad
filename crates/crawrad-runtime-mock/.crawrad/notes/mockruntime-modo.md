---
id: mockruntime-modo
kind: doc
target_file: src/lib.rs
line_hint: 99
author: 
created: 
title: "MockRuntime::modo"
tags: []
signature: "pub fn modo (& self) -> crawrad_runtime :: Mode"
---

Design or live, as the adapter received it.

The trait's `set_modo` has an EMPTY default body — a runtime that does not yet
know how to be live continues drawing. That is why storing it here is not a luxury: without
this, a `Preview` test would only assert that a harness `bool`
flipped, and would pass with the runtime call removed.

# Examples

```rust
use crawrad_runtime_mock::MockRuntime;
use crawrad_runtime::Mode;

let rt = MockRuntime::new();
assert_eq!(rt.modo(), Mode::Design);
```
