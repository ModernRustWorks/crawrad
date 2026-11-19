---
id: targets-egui-classes
kind: doc
target_file: src/targets/egui/mod.rs
line_hint: 47
author: 
created: 
title: "CLASSES"
tags: []
signature: "pub static CLASSES : std :: sync :: LazyLock < Vec < & 'static str > > = std :: sync :: LazyLock :: new (components :: classes)"
---

The classes that the egui generator emits.

**The classes that this target emits — derived from what it REGISTERED.**

It used to come from `ComponentClass::ALL_STR`, the catalog's closed `enum`: the target
claimed to generate the sixteen variants of the enum, rather than those it actually
registered. They coincided by luck, and [ADR 0031] measured the cost of the enum —
**as long as a class was not part of it, no target could register a handler
for it**.

Derived from the target's own registry, it is true by construction, and
class 27 — which comes from a file (ADR 0033) — appears here without anyone
editing any list. It is the same grammar as `CLASSES` in the Makepad adapter.


# Examples

```rust
use crawrad_codegen::targets::egui::CLASSES;

assert!(CLASSES.contains(&"Button"));
```
