---
id: targets-makepad-classes
kind: doc
target_file: src/targets/makepad/mod.rs
line_hint: 51
author: 
created: 
title: "CLASSES"
tags: []
signature: "pub static CLASSES : std :: sync :: LazyLock < Vec < & 'static str > > = std :: sync :: LazyLock :: new (components :: classes)"
---

The classes that this target emits, derived from what it registered.

**The classes that this target emits — derived from what it REGISTERED.**

It used to come from `ComponentClass::ALL_STR`, the closed `enum` of the catalog: the target
claimed to generate the sixteen variants of the enum, and not those it actually
registered. They coincided by luck, and [ADR 0031] measured the cost of the enum —
**until a class entered it, no target could register a handler
for it**.

Derived from the target's own registry, it is true by construction, and
class 27 — which comes from a file (ADR 0033) — appears here without anyone
editing any list. It is the same grammar as the Makepad adapter's `CLASSES`.


# Examples

```rust
use crawrad_codegen::targets::makepad::CLASSES;

assert!(CLASSES.contains(&"Button"));
```
