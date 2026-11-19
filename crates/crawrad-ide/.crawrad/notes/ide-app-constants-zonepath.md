---
id: ide-app-constants-zonepath
kind: doc
target_file: src/ide_app/constants.rs
line_hint: 29
author: 
created: 
title: "ZonePath"
tags: []
signature: "pub (super) type ZonePath = fn () -> & 'static [LiveId]"
---

The path of a widget in the DSL, deferred until invocation.

An alias rather than the expanded type: without it, `ALIGN_ZONES` below becomes a
`[(fn() -> &'static [LiveId], &str); 5]` written in the declaration, which is
exactly `clippy::type_complexity` — and the house rule is to fix the
lint, never silence it.
