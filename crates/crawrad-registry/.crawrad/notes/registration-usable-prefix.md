---
id: registration-usable-prefix
kind: doc
target_file: src/registration.rs
line_hint: 90
author: 
created: 
title: "usable_prefix"
tags: []
signature: "fn usable_prefix (s : & str) -> bool"
---

Returns whether a string is a valid identifier prefix for field names.

**Is this a prefix the automatic name can be built from?**

`btn` becomes `btn_ok`, which is a field of the generated struct — so it
obeys the rule a Rust field obeys.
