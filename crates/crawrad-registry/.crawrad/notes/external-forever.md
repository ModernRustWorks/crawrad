---
id: external-forever
kind: doc
target_file: src/external.rs
line_hint: 50
author: 
created: 
title: "forever"
tags: []
signature: "fn forever (s : String) -> & 'static str"
---

Gives a runtime string the `'static` life the catalogue requires.

One function, so the whole leak of this crate is at one address — see the
module header for why it is bounded.
