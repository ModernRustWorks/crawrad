---
id: gerador-classes
kind: doc
target_file: src/lib.rs
line_hint: 166
author: 
created: 
title: "Gerador::classes"
tags: []
signature: "fn classes (& self) -> & 'static [& 'static str]"
---

The classes this generator **actually** emits.

# Examples

```rust
use crawrad_codegen::{Gerador, GeradorRxp};

let gerador = GeradorRxp;
assert!(gerador.classes().contains(&"Button"));
```

The same reason as `Runtime::classes`,
one step further: a class that the designer draws and the generator does not
emit turns into a `TODO` in user code — and the user only discovers this
when compiling the generated project.
