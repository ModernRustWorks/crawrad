---
id: targets-makepad-components-codegen-enterkey
kind: doc
target_file: src/targets/makepad/components/codegen.rs
line_hint: 88
author: 
created: 
title: "EnterKey"
tags: []
signature: "pub enum EnterKey"
---

Behavior of the Enter key in a control's text box.

What Enter does in a control's text box (see
[`MakepadComponentCodegen::enter_key`]).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::codegen::EnterKey;

let k = EnterKey::Untouched;
assert_eq!(k, EnterKey::Untouched);
```
