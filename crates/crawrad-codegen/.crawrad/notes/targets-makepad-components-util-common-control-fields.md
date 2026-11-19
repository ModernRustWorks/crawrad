---
id: targets-makepad-components-util-common-control-fields
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 22
author: 
created: 
title: "common_control_fields"
tags: []
signature: "pub fn common_control_fields () -> Vec < String >"
---

Returns the standard common control fields (`visible` and `enabled`).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::common_control_fields;

let fields = common_control_fields();
assert_eq!(fields.len(), 2);
```
