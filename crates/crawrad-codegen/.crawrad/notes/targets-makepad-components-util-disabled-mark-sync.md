---
id: targets-makepad-components-util-disabled-mark-sync
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 136
author: 
created: 
title: "disabled_mark_sync"
tags: []
signature: "pub fn disabled_mark_sync (field : & str , widget : & str) -> String"
---

Emits synchronization code for disabled check box or radio button marks.

A disabled CheckBox / RadioButton shows its mark only when it is checked
(#431: Makepad 1.0.0 draws the disabled mark whatever the state — see
`crawrad_runtime_makepad::enabled::disabled_mark`).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::disabled_mark_sync;

let code = disabled_mark_sync("chk1", "chk1_widget");
assert!(code.contains("disabled_mark"));
```
