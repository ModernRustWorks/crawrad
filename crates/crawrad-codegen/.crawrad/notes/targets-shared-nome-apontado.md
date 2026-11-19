---
id: targets-shared-nome-apontado
kind: doc
target_file: src/targets/shared.rs
line_hint: 89
author: 
created: 
title: "nome_apontado"
tags: []
signature: "pub fn nome_apontado (doc : & crawrad_ir :: Document , id : & str) -> Option < String >"
---

Converts a reference into the name of the pointed component.

**A REFERENCE becomes the NAME of the pointed component.**

The document stores a `Value::Ref` with the target's **id**, which is an address and
never reaches the interface (ADR 0008). What the generated application receives is the
NAME — the same one the user sees in the Object Inspector and writes in the handler.

# The hole this closes, and it was a big one

Until 18/09/2026 **neither of the two targets** knew what to do with a
`Value::Ref`: the vocabulary emitted `Set` (see [`crate::emission`], which already
orders the target's `Create` beforehand), and both discarded it silently.

The effect appeared fully in the new connection design: the user chose the
engine in the Inspector, the designer saved it, and the application was born with the field
**empty**. The choice died between the click and `cargo build`.

# The limit, declared

A `String` comes out, because that is what the field of a component declaring
`kind = "reference"` is today. A TYPED reference — the application holding the
other actual component — requires the manifest to state the field's type, and
that does not exist. See ADR 0040.

# Examples

```rust
use crawrad_codegen::targets::shared::nome_apontado;
use crawrad_ir::Document;

let doc = Document::new("form", "Form");
assert_eq!(nome_apontado(&doc, ""), None);
```
