---
id: contract-targets
kind: doc
target_file: src/contract.rs
line_hint: 16
author: 
created: 
title: "TARGETS"
tags: []
signature: "pub const TARGETS : & [& str] = & [\"win32\" , \"egui\" , \"makepad\"]"
---

Known runtime targets. A property prefix must be one of them.

# Examples

```rust
use crawrad_ir::contract::TARGETS;

assert!(TARGETS.contains(&"egui"));
assert!(TARGETS.contains(&"makepad"));
```

# `makepad` entered on 2026-09-17, and its absence was COSTLY

[ADR 0031] measured and named it: as long as `makepad` was not here,
`PropKey::parse("makepad.x")` returned `UnknownTarget` and `validate()`
failed. **Every Makepad-specific property was forced into common or
into nothing** — and that was how `ScrollBox.horz_scroll_bar`,
`ScrollBox.vert_scroll_bar` and `border_width` ended up in `COMMON`, read
only by Makepad files.

The law of [ADR 0002] states that *a property only enters the common core if
every target honors it*. It was being broken with a green suite, because the
escape valve was closed.

# And `win32` remains here, dead, ON PURPOSE

RCL departed in [ADR 0013] and no runtime has honored `win32.*` since then. It
stays because **files saved before that day still carry it**: the
`form_main.crawform` of the examples has eight `win32.*` keys right now,
and removing the target from the list would cause `validate()` to reject the owner's
own form.

Two of them — `PasswordChar` and `readonly` — have already moved up to common and are
dead duplicates. The other six (`FullRowSelect`, `HotTrack`,
`ShowButtons`, `ShowLines`, `ShowRoot` and `default`) are classic RAD IDE features
that **this catalog does not yet have**: deleting them silently would lose the
only notation that they are missing.

Removing `win32` is a format migration plus six properties to declare, and
is tracked as its own task. It does not block anything: adding a
target only EXPANDS what is accepted.

