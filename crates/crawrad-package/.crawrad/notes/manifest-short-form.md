---
id: manifest-short-form
kind: doc
target_file: src/manifest.rs
line_hint: 978
author: 
created: 
title: "short_form"
tags: []
signature: "fn short_form (v : & Value) -> Option < JsonValue >"
---

**The value in SHORT form** — `None` when it does not exist for it.

Does not exist for the variants that no `kind` names (`Font`, `Size`,
`Point`, `Map`, `Null`): without a `kind` to specify the type, the short form
would be ambiguous on reading.
