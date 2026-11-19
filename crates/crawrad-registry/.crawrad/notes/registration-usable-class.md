---
id: registration-usable-class
kind: doc
target_file: src/registration.rs
line_hint: 80
author: 
created: 
title: "usable_class"
tags: []
signature: "fn usable_class (s : & str) -> bool"
---

Returns whether a string is a valid Rust type identifier for a class.

**Is this a class name the generated project can carry?**

It becomes a Rust type in the generated code, so it obeys the rule a Rust
type obeys. The check is written here and not left to `rustc` because the
error has to arrive at install time, with the component's name on it — and
not as a compile error inside a project the user did not write.
