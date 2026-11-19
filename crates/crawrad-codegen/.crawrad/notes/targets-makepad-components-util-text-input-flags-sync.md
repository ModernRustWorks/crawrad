---
id: targets-makepad-components-util-text-input-flags-sync
kind: doc
target_file: src/targets/makepad/components/util.rs
line_hint: 145
author: 
created: 
title: "text_input_flags_sync"
tags: []
signature: "pub fn text_input_flags_sync (field : & str , password : bool) -> String"
---

Emits synchronization code for text input password and read-only flags.

The sync that keeps those two in step with the form's fields at run time
(`password` when the component has a `password_char`).

# Examples

```rust
use crawrad_codegen::targets::makepad::components::util::text_input_flags_sync;

let code = text_input_flags_sync("edit1", true);
assert!(code.contains("is_password"));
assert!(code.contains("is_read_only"));
```
