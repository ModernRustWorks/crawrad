---
id: manifest-read-value
kind: doc
target_file: src/manifest.rs
line_hint: 1042
author: 
created: 
title: "read_value"
tags: []
signature: "fn read_value (kind : Kind , t : & JsonValue) -> Option < Value >"
---

Reads a property value, accepting both short and long forms.

**Reads the value, accepting both forms.**

The rule is a single one: a table with a text key `t` is the long form. No
short form has `t` — see the header.
