---
id: manifest-number
kind: doc
target_file: src/manifest.rs
line_hint: 1011
author: 
created: 
title: "number"
tags: []
signature: "fn number (t : & JsonValue) -> Option < f64 >"
---

Extracts an `f64` from a JSON number.

A JSON number as `f64` — integer works, and JSON requires it: `0.0` written
as `0` arrives as integer, and refusing it would reject a correctly written file.
