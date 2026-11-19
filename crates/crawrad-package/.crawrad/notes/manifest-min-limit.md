---
id: manifest-min-limit
kind: doc
target_file: src/manifest.rs
line_hint: 1016
author: 
created: 
title: "min_limit"
tags: []
signature: "fn min_limit (k : & ExternalKind) -> Option < JsonValue >"
---

Returns the lowest value the type accepts, if explicitly bounded.

The lowest value the type accepts, when it has one — and `None` when the
limit is that of the type itself, which is what the catalogue uses by default.

Writing `min = -9223372036854775808` on every integer property would be
noise that hides the only `min = 1` that was truly decided.
