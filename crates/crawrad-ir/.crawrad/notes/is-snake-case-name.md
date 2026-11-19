---
id: is-snake-case-name
kind: doc
target_file: src/lib.rs
line_hint: 592
author: 
created: 
title: "is_snake_case_name"
tags: []
signature: "pub fn is_snake_case_name (name : & str) -> bool"
---

The `Name` is a snake_case Rust identifier — `btn_ok`, and not `btnOK`.

# Examples

```rust
use crawrad_ir::is_snake_case_name;

assert!(is_snake_case_name("btn_ok"));
assert!(!is_snake_case_name("btnOK"));
assert!(!is_snake_case_name("1btn"));
```

# Why this is an INPUT rule, and not an output fix

A `Name` becomes a field (`pub btn_ok: Button`) and method in the generated project. A
`btnOK` comes out `self.btnOK`, and `rustc` reports `non_snake_case` in a file
that the user did not write and cannot fix — the `.designer.rs` is
rewritten entirely on the next generation.

There were two options, and the owner chose on 2026-09-16: *"this is Rust, not
the classic RAD IDE. I don't want to help the classic RAD IDE dev come write as if they were in
the classic RAD IDE and have the IDE cope with it. YOU MUST WRITE AS RUST ALLOWS."*

The rejected option was silent translation — the Makepad target converted the
name before emitting, egui did not convert, and the name that the user read in the
Inspector was not the name their code had. **The one that stayed is this: the
name that the user sees is the name the code has, and whatever does not work is
rejected upfront, with the correct form suggested** (see [`snake_case_hint`]).

The third option — `#[allow(non_snake_case)]` — was not on the table: it was
already tried here, was internal, silenced the user's entire file, and
hid eighteen defects.

# The scope, and why it fits in `crawrad-ir`

`name_has_alphanumeric` right above says that the Rust-specific rule
"remains in `crawrad-codegen`". It remains — what `identifier()` enforces
(reserved keywords, for example) stays there, and keeps rejecting before
writing a file. What lives here is what **every target in this project** has
in common: both targets are Rust, and both derive a field from `Name`.
A rule that applies to every target is a document rule, and it is the one that
`Command::Rename` can enforce **before** the name enters — which is the
only moment where telling the user still helps.
