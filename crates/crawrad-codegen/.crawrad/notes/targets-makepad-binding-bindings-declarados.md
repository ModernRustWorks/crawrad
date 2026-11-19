---
id: targets-makepad-binding-bindings-declarados
kind: doc
target_file: src/targets/makepad/binding.rs
line_hint: 278
author: 
created: 
title: "BINDINGS_DECLARADOS"
tags: []
signature: "pub const BINDINGS_DECLARADOS : & [(& str , Binding)] = & [(\"on_click\" , Binding { accessor : \"view\" , part : \"\" , query : \"finger_up\" , shape : Shape :: Option , writeback : None , } ,)]"
---

Event bindings table for declared components.

**The `Bind` of a DECLARED component** — derived, not written in the
manifest.

# Why it is not declarable, and this is a decision

A third-party component **is not a framework widget**: it is a `<View>`
that paints itself using verbs from `manifest.json`. It has neither `button` nor
`text_input` to query — the only surface it has is the **rectangle itself**.

Allowing the manifest to write `accessor = "button"` would mean asking the component
author to know Makepad's widget vocabulary, and tomorrow that of the
next target. It is option A of
ADR 0036,
rejected: *of the eight precedents measured, zero ask the component author
to write target-language text.*

So the declared table is ONE, holds for every class that comes from outside,
and grows when one more event can be honestly handled.

# Examples

```rust
use crawrad_codegen::targets::makepad::binding::BINDINGS_DECLARADOS;

assert!(!BINDINGS_DECLARADOS.is_empty());
```
