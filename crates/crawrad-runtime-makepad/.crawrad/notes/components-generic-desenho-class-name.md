---
id: components-generic-desenho-class-name
kind: doc
target_file: src/components/generic.rs
line_hint: 24
author: 
created: 
title: "Desenho::class_name"
tags: []
signature: "fn class_name (& self) -> & 'static str"
---

Returns an empty string as it answers for no specific class.

It answers for no class at all: whoever reaches it is the dispatch's
`unwrap_or`, and never a name comparison.
