---
id: makepadruntime-com-desenhos
kind: doc
target_file: src/lib.rs
line_hint: 321
author: 
created: 
title: "MakepadRuntime::com_desenhos"
tags: []
signature: "pub fn com_desenhos (& mut self , desenhos : crawrad_ir :: drawing :: DeclaredLooks)"
---

Configures the declared drawings per class for this adapter.

**Teaches this adapter how DECLARED classes paint themselves.**

The shell calls this once, with whatever its catalog brings from installed
`manifest.json` files. A class without an entry here continues falling back to
code drawing, and whatever has neither continues falling back to
dashed outline — the `unwrap_or` chain does not change.

The same call carries the inheriting classes' bases (#934): a `PriceEdit` that
inherits `SpinEdit` is painted as a `SpinEdit`.

# Examples

```rust
use crawrad_runtime_makepad::MakepadRuntime;

let mut rt = MakepadRuntime::new();
rt.com_desenhos(crawrad_ir::drawing::DeclaredLooks::default());
```
