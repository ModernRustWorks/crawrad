---
id: targets-makepad-components-util-value-to-rust-literal
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 62
author: 
created: 
title: "value_to_rust_literal"
tags: []
signature: "pub fn value_to_rust_literal (val : & Value) -> String"
---

Converts a `Value` to its Rust literal code representation.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::value_to_rust_literal;
use crawrad_ir::Value;

assert_eq!(value_to_rust_literal(&Value::Int(42)), "42");
```

# Panics

Panics if a font value cannot be formatted into a code literal.
