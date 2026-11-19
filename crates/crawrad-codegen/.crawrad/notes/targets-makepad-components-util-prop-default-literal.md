---
id: targets-makepad-components-util-prop-default-literal
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 117
author: 
created: 
title: "prop_default_literal"
tags: []
signature: "pub fn prop_default_literal (prop : & PropDesc) -> String"
---

Converts a property's default value to its Rust literal representation.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::prop_default_literal;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let desc = reg.get("Button").unwrap();
let prop = &desc.props[0];
assert!(!prop_default_literal(prop).is_empty());
```
