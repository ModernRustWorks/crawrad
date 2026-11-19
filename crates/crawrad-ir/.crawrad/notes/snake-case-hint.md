---
id: snake-case-hint
kind: doc
target_file: src/lib.rs
line_hint: 601
author: 
created: 
title: "snake_case_hint"
tags: []
signature: "pub fn snake_case_hint (name : & str) -> Option < String >"
---

The `snake_case` of `name`, when one can be proposed — `btnOK` → `btn_ok`.

# Examples

```rust
use crawrad_ir::snake_case_hint;

assert_eq!(snake_case_hint("btnOK"), Some("btn_ok".to_string()));
assert_eq!(snake_case_hint("btn_ok"), None);
```

# This function was once the translator, and that difference is everything

The body here is that of the former `to_snake_case`, which existed **twice**,
character by character, in `crawrad-codegen` and in `crawrad-registry`. It ran
under the hood: the user wrote `btnOK`, the Makepad target emitted `btn_ok`, and
nobody said anything. Here it fixes nothing — it only **provides the
advice that accompanies the rejection**, and whoever types decides.

Returns `None` when there is no honest advice to give: the name is already right,
or whatever would come out of conversion is still not an identifier (`my button`
becomes `my_button`, which `rustc` rejects just the same). Rejection without
suggestion still says what to do; wrong suggestion sends the user banging
into the same door.
